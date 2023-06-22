use std::collections::HashMap;

use clap::Args;
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
    #[clap(long, env = "BLITZ_TANKS_MODEL_ENABLE_DAMPING")]
    pub enable_damping: bool,

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
        let biases = Self::calculate_biases(&votes);
        let mut similarities = self.calculate_similarities(&mut votes, &biases);
        Self::sort_similarities(&mut similarities);
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

    #[must_use]
    fn calculate_biases<'a>(votes: &'a HashMap<u16, Vec<&'a Vote>>) -> HashMap<u16, f64> {
        votes
            .par_iter()
            .map(|(tank_id, votes)| {
                let bias = votes.iter().map(|vote| f64::from(vote.rating)).sum::<f64>()
                    / votes.len() as f64;
                (*tank_id, bias)
            })
            .collect()
    }

    fn calculate_similarities(
        &self,
        votes: &mut HashMap<u16, Vec<&Vote>>,
        biases: &HashMap<u16, f64>,
    ) -> HashMap<u16, Box<[(u16, f64)]>> {
        Self::sort_votes(votes);
        biases
            .par_iter()
            .map(|(i, bias_i)| {
                let similar: Box<[(u16, f64)]> = biases
                    .iter()
                    .filter(|(j, _)| *i != **j)
                    .map(|(j, bias_j)| {
                        // FIXME: I do the same calculation twice: for `(i, j)` and `(j, i)`.
                        let similarity =
                            self.calculate_similarity(*bias_i, &votes[i], *bias_j, &votes[j]);
                        (*j, similarity)
                    })
                    .collect();
                (*i, similar)
            })
            .collect()
    }

    fn calculate_similarity(
        &self,
        bias_i: f64,
        votes_i: &[&Vote],
        bias_j: f64,
        votes_j: &[&Vote],
    ) -> f64 {
        let (numerator, denominator_i, denominator_j) =
            merge_join_by(votes_i, votes_j, |i, j| i.account_id.cmp(&j.account_id)).fold(
                (0.0, 0.0, 0.0),
                |(mut numerator, mut denominator_i, mut denominator_j), either| {
                    match either {
                        EitherOrBoth::Left(vote_i) => {
                            if self.enable_damping {
                                denominator_i += (vote_i.rating - bias_i).powi(2);
                            }
                        }
                        EitherOrBoth::Right(vote_j) => {
                            if self.enable_damping {
                                denominator_j += (vote_j.rating - bias_j).powi(2);
                            }
                        }
                        EitherOrBoth::Both(vote_i, vote_j) => {
                            let diff_i = vote_i.rating - bias_i;
                            let diff_j = vote_j.rating - bias_j;
                            numerator += diff_i * diff_j;
                            denominator_i += diff_i.powi(2);
                            denominator_j += diff_j.powi(2);
                        }
                    }
                    (numerator, denominator_i, denominator_j)
                },
            );

        if numerator.abs() >= f64::EPSILON
            && denominator_i >= f64::EPSILON
            && denominator_j >= f64::EPSILON
        {
            numerator / denominator_i.sqrt() / denominator_j.sqrt()
        } else {
            0.0
        }
    }

    /// Sort each vehicle's similar vehicles by decreasing similarity.
    fn sort_similarities(similarities: &mut HashMap<u16, Box<[(u16, f64)]>>) {
        for similar in similarities.values_mut() {
            similar.sort_unstable_by(|(_, lhs), (_, rhs)| rhs.total_cmp(lhs))
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
    biases: HashMap<u16, f64>,

    /// Mapping from vehicle's tank ID to other vehicles' similarities.
    #[serde_as(as = "Vec<(_, _)>")]
    similarities: HashMap<u16, Box<[(u16, f64)]>>,
}

impl Model {
    #[must_use]
    #[instrument(skip_all, fields(target_id = target_id))]
    pub fn predict(&self, target_id: u16, source_ratings: &HashMap<u16, Rating>) -> Option<f64> {
        let similar = self.similarities.get(&target_id)?;
        let (sum, weight) = similar
            .iter()
            .filter(|(_, similarity)| self.params.include_negative || (*similarity > 0.0))
            .filter_map(|(tank_id, similarity)| {
                source_ratings
                    .get(tank_id)
                    .map(|rating| (*similarity, self.biases[tank_id], f64::from(*rating)))
            })
            .take(self.params.n_neighbors)
            .fold((0.0, 0.0), |(sum, weight), (similarity, similar_bias, similar_rating)| {
                (sum + similarity * (similar_rating - similar_bias), weight + similarity.abs())
            });
        if weight >= f64::EPSILON {
            Some(self.biases[&target_id] + sum / weight)
        } else {
            None
        }
    }

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

#[derive(Serialize, Deserialize)]
pub struct Vehicle {
    /// Average vehicle rating.
    #[serde(rename = "b")]
    pub bias: f64,

    /// Similar vehicles (tank ID and similarity), sorted by descending similarity.
    #[serde(rename = "s")]
    pub similar: Box<[(u16, f64)]>,
}
