use std::collections::HashMap;

use clap::Args;
use itertools::{merge_join_by, EitherOrBoth, Itertools};
use mongodb::bson::serde_helpers;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    models::{rating::Rating, vote::Vote},
    prelude::*,
    trainer::prediction::Prediction,
};

/// Model parameters.
#[derive(Debug, Args)]
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

/// Item-item kNN collaborative filtering.
#[must_use]
#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
pub struct Model {
    /// Mapping from vehicle's tank ID to other vehicles' similarities.
    ///
    /// # Note
    ///
    /// The mapping values only contain entries,
    /// for which tank ID are **greater** than the respective mapping key.
    #[serde_as(as = "Vec<(_, _)>")]
    vehicles: HashMap<u16, Vehicle>,

    n_neighbors: usize,

    include_negative: bool,

    #[serde(with = "serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime,
}

impl Model {
    pub fn fit(votes: &[Vote], params: &Params) -> Self {
        let votes = votes.iter().into_group_map_by(|vote| vote.tank_id);
        let biased = Self::calculate_biases(&votes);
        let mut vehicles = Self::calculate_similarities(&biased, params);
        Self::sort(&mut vehicles);
        Model {
            vehicles,
            n_neighbors: params.n_neighbors,
            include_negative: params.include_negative,
            created_at: Utc::now(),
        }
    }

    #[must_use]
    #[instrument(skip_all, fields(target_id = target_id))]
    pub fn predict(&self, target_id: u16, source_ratings: &HashMap<u16, Rating>) -> Option<f64> {
        let target_vehicle = self.vehicles.get(&target_id)?;
        let (sum, weight) = target_vehicle
            .similar
            .iter()
            .filter(|similar_vehicle| self.include_negative || (similar_vehicle.similarity > 0.0))
            .filter_map(|similar_vehicle| {
                source_ratings
                    .get(&similar_vehicle.tank_id)
                    .map(|rating| (similar_vehicle, f64::from(*rating)))
            })
            .take(self.n_neighbors)
            .fold((0.0, 0.0), |(sum, weight), (similar_vehicle, similar_rating)| {
                (
                    sum + similar_vehicle.similarity * (similar_rating - similar_vehicle.bias),
                    weight + similar_vehicle.similarity.abs(),
                )
            });
        if weight >= f64::EPSILON {
            Some(target_vehicle.bias + sum / weight)
        } else {
            None
        }
    }

    pub fn predict_many<'a>(
        &'a self,
        target_ids: impl IntoIterator<Item = u16> + 'a,
        source_ratings: &'a HashMap<u16, Rating>,
    ) -> impl Iterator<Item = Prediction> + 'a {
        target_ids.into_iter().filter_map(|target_id| {
            self.predict(target_id, source_ratings)
                .map(|rating| Prediction { tank_id: target_id, rating })
        })
    }

    #[must_use]
    fn calculate_biases<'a>(votes: &'a HashMap<u16, Vec<&'a Vote>>) -> Box<[Biased<'a>]> {
        votes
            .par_iter()
            .map(|(tank_id, votes)| {
                let bias = votes.iter().map(|vote| f64::from(vote.rating)).sum::<f64>()
                    / votes.len() as f64;
                Biased {
                    tank_id: *tank_id,
                    bias,
                    votes: votes.as_ref(),
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn calculate_similarities(biased: &[Biased], params: &Params) -> HashMap<u16, Vehicle> {
        biased
            .par_iter()
            .map(|vehicle_i| {
                let similar: Box<[SimilarVehicle]> = biased
                    .iter()
                    .filter(|vehicle_j| vehicle_j.tank_id != vehicle_i.tank_id)
                    .filter_map(|vehicle_j| {
                        // FIXME: I do the same calculation twice: for `(i, j)` and `(j, i)`.
                        Self::calculate_similarity(vehicle_i, vehicle_j, params).map(|similarity| {
                            SimilarVehicle {
                                similarity,
                                tank_id: vehicle_j.tank_id,
                                bias: vehicle_j.bias,
                            }
                        })
                    })
                    .collect();
                let entry = Vehicle { bias: vehicle_i.bias, similar };
                (vehicle_i.tank_id, entry)
            })
            .collect()
    }

    fn calculate_similarity(
        vehicle_i: &Biased,
        vehicle_j: &Biased,
        params: &Params,
    ) -> Option<f64> {
        let (numerator, denominator_i, denominator_j) =
            merge_join_by(vehicle_i.votes, vehicle_j.votes, |i, j| i.account_id.cmp(&j.account_id))
                .fold(
                    (0.0, 0.0, 0.0),
                    |(mut numerator, mut denominator_i, mut denominator_j), either| {
                        match either {
                            EitherOrBoth::Left(vote_i) => {
                                if params.enable_damping {
                                    denominator_i += (vote_i.rating - vehicle_i.bias).powi(2);
                                }
                            }
                            EitherOrBoth::Right(vote_j) => {
                                if params.enable_damping {
                                    denominator_j += (vote_j.rating - vehicle_j.bias).powi(2);
                                }
                            }
                            EitherOrBoth::Both(vote_i, vote_j) => {
                                let diff_i = vote_i.rating - vehicle_i.bias;
                                let diff_j = vote_j.rating - vehicle_j.bias;
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
            Some(numerator / denominator_i.sqrt() / denominator_j.sqrt())
        } else {
            None
        }
    }

    fn sort(vehicles: &mut HashMap<u16, Vehicle>) {
        for entry in vehicles.values_mut() {
            entry
                .similar
                .sort_unstable_by(|lhs, rhs| rhs.similarity.total_cmp(&lhs.similarity))
        }
    }
}

struct Biased<'a> {
    tank_id: u16,

    /// Average vehicle rating.
    bias: f64,

    votes: &'a [&'a Vote],
}

#[derive(Serialize, Deserialize)]
pub struct Vehicle {
    /// Average vehicle rating.
    #[serde(rename = "b")]
    pub bias: f64,

    /// Similar vehicles (tank ID and similarity), sorted by descending similarity.
    #[serde(rename = "s")]
    pub similar: Box<[SimilarVehicle]>,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
pub struct SimilarVehicle {
    /// Similar vehicle ID.
    #[serde(rename = "i")]
    #[serde_as(as = "serde_with::TryFromInto<i32>")]
    tank_id: u16,

    /// Similarity of this vehicle to the source vehicle.
    #[serde(rename = "w")]
    similarity: f64,

    /// The similar vehicle's mean rating.
    #[serde(rename = "b")]
    bias: f64,
}
