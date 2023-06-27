use std::collections::HashMap;

use clap::Args;
use indexmap::IndexMap;
use itertools::{merge_join_by, EitherOrBoth, Itertools};
use mongodb::bson::serde_helpers;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    models::{rating::Rating, vote::Vote},
    prelude::*,
};

/// Model parameters.
#[derive(Debug, Args, Serialize, Deserialize, Copy, Clone)]
pub struct Params {
    #[clap(long, env = "BLITZ_TANKS_MODEL_NEIGHBORS")]
    /// Number of top similar vehicles to include in a prediction.
    pub n_neighbors: usize,

    /// Include negative similarities.
    #[clap(long, env = "BLITZ_TANKS_MODEL_INCLUDE_NEGATIVE")]
    pub include_negative: bool,
}

impl Params {
    pub fn fit(self, votes: &[Vote]) -> Model {
        let mut votes = votes.iter().into_group_map_by(|vote| vote.tank_id);
        Self::sort_votes(&mut votes);
        let biases = Self::calculate_biases(&votes);
        let similarities = Self::calculate_similarities(&votes, &biases);
        Model {
            created_at: Utc::now(),
            params: self,
            biases,
            similarities,
        }
    }

    /// Sort each vehicle's entries by account ID, to prepare for `merge_join_by()`.
    fn sort_votes(votes: &mut HashMap<u16, Vec<&Vote>>) {
        for votes in votes.values_mut() {
            votes.sort_by_key(|vote| vote.account_id);
        }
    }

    /// Calculate mean rating for each vehicle.
    ///
    /// # Returns
    ///
    /// Mapping from tank ID to its mean rating.
    #[must_use]
    fn calculate_biases<'a>(votes: &'a HashMap<u16, Vec<&'a Vote>>) -> IndexMap<u16, f64> {
        let mut biases: IndexMap<_, _> = votes
            .par_iter()
            .map(|(tank_id, votes)| {
                let bias = votes.iter().map(|vote| f64::from(vote.rating)).sum::<f64>()
                    / votes.len() as f64;
                (*tank_id, bias)
            })
            .collect();
        biases.sort_unstable_by(|_, lhs, _, rhs| rhs.total_cmp(lhs));
        biases
    }

    /// Calculate similarities between different vehicles.
    ///
    /// # Returns
    ///
    /// Mapping from a tank ID to a list of other vehicles, the latter are sorted
    /// by decreasing similarity in respect to the former.
    #[must_use]
    fn calculate_similarities(
        votes: &HashMap<u16, Vec<&Vote>>,
        biases: &IndexMap<u16, f64>,
    ) -> HashMap<u16, Box<[(u16, f64)]>> {
        let mut similarities: HashMap<_, _> = biases
            .par_iter()
            .map(|(i, bias_i)| {
                let similarities: Box<[(u16, f64)]> = biases
                    .iter()
                    .filter(|(j, _)| *i != **j)
                    .map(|(j, bias_j)| {
                        // FIXME: I do the same calculation twice: for `(i, j)` and `(j, i)`.
                        (*j, Self::calculate_similarity(*bias_i, &votes[i], *bias_j, &votes[j]))
                    })
                    .collect();
                (*i, similarities)
            })
            .collect();
        for similar in similarities.values_mut() {
            similar.sort_unstable_by(|(_, lhs), (_, rhs)| rhs.total_cmp(lhs))
        }
        similarities
    }

    /// Calculate similarity between two vehicles, specified by their respective biases
    /// and votes sorted by account ID.
    fn calculate_similarity(bias_i: f64, votes_i: &[&Vote], bias_j: f64, votes_j: &[&Vote]) -> f64 {
        let mut dot_product = 0.0;
        let mut norm2_i = 0.0;
        let mut norm2_j = 0.0;

        for either in merge_join_by(votes_i, votes_j, |i, j| i.account_id.cmp(&j.account_id)) {
            match either {
                EitherOrBoth::Left(vote_i) => {
                    norm2_i += (vote_i.rating - bias_i).powi(2);
                }
                EitherOrBoth::Right(vote_j) => {
                    norm2_j += (vote_j.rating - bias_j).powi(2);
                }
                EitherOrBoth::Both(vote_i, vote_j) => {
                    let diff_i = vote_i.rating - bias_i;
                    let diff_j = vote_j.rating - bias_j;
                    dot_product += diff_i * diff_j;
                    norm2_i += diff_i.powi(2);
                    norm2_j += diff_j.powi(2);
                }
            }
        }

        if norm2_i >= f64::EPSILON && norm2_j >= f64::EPSILON {
            dot_product / norm2_i.sqrt() / norm2_j.sqrt()
        } else {
            0.0
        }
    }
}

/// Item-item kNN collaborative filtering.
#[must_use]
#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
pub struct Model {
    #[serde(with = "serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime,

    params: Params,

    #[serde_as(as = "Vec<(_, _)>")]
    pub biases: IndexMap<u16, f64>,

    /// Mapping from vehicle's tank ID to other vehicles' similarities.
    #[serde_as(as = "Vec<(_, _)>")]
    similarities: HashMap<u16, Box<[(u16, f64)]>>,
}

impl Model {
    #[cfg(test)]
    pub fn empty() -> Self {
        Self {
            created_at: Utc::now(),
            params: Params {
                n_neighbors: 0,
                include_negative: false,
            },
            biases: Default::default(),
            similarities: Default::default(),
        }
    }

    #[must_use]
    #[instrument(skip_all, fields(target_id = target_id))]
    pub fn predict(&self, target_id: u16, source_ratings: &HashMap<u16, Rating>) -> Option<f64> {
        let (sum, weight) = self
            .similarities
            .get(&target_id)?
            .iter()
            .filter(|(_, similarity)| self.params.include_negative || (*similarity > 0.0))
            .filter_map(|(tank_id, similarity)| {
                source_ratings
                    .get(tank_id)
                    .map(|rating| (similarity, rating.into_f64() - self.biases[tank_id]))
            })
            .take(self.params.n_neighbors)
            .fold((0.0, 0.0), |(sum, weight), (similarity, relative_rating)| {
                (sum + similarity * relative_rating, weight + similarity.abs())
            });

        if weight >= f64::EPSILON {
            Some(self.biases[&target_id] + sum / weight)
        } else {
            None
        }
    }

    #[instrument(skip_all)]
    pub fn predict_many<'a>(
        &'a self,
        target_ids: impl IntoIterator<Item = u16> + 'a,
        source_ratings: &'a HashMap<u16, Rating>,
    ) -> impl Iterator<Item = (u16, f64)> + 'a {
        target_ids.into_iter().filter_map(|target_id| {
            self.predict(target_id, source_ratings).map(|rating| (target_id, rating))
        })
    }
}
