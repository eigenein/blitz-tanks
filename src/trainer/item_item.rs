use std::collections::HashMap;

use itertools::{merge_join_by, EitherOrBoth, Itertools};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    models::{rating::Rating, vote::Vote},
    prelude::*,
    trainer::prediction::Prediction,
};

/// Model parameters.
#[derive(Default, Debug)]
pub struct Params {
    pub disable_damping: bool,

    /// Number of top similar vehicles to include in a prediction.
    pub n_neighbors: usize,

    /// Include negative similarities.
    pub include_negative: bool,
}

/// Item-item kNN collaborative filtering.
#[must_use]
#[derive(Serialize, Deserialize)]
pub struct Model {
    /// Mapping from vehicle's tank ID to other vehicles' similarities.
    ///
    /// # Note
    ///
    /// The mapping values only contain entries,
    /// for which tank ID are **greater** than the respective mapping key.
    vehicles: HashMap<i32, Vehicle>,

    n_neighbors: usize,

    include_negative: bool,
}

impl Model {
    pub fn fit(votes: &[Vote], params: &Params) -> Self {
        let votes = votes.iter().into_group_map_by(|vote| vote.tank_id);
        let biased = Self::calculate_biases(&votes);
        let mut vehicles = Self::calculate_similarities(&biased, params.disable_damping);
        Self::sort(&mut vehicles);
        Model {
            vehicles,
            n_neighbors: params.n_neighbors,
            include_negative: params.include_negative,
        }
    }

    #[must_use]
    #[instrument(skip_all, fields(target_id = target_id))]
    pub fn predict(&self, target_id: i32, source_ratings: &HashMap<i32, Rating>) -> Option<f64> {
        let target_vehicle = self.vehicles.get(&target_id)?;
        let (numerator, denominator) = target_vehicle
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
                    sum + similar_vehicle.similarity
                        * (similar_rating - similar_vehicle.mean_rating),
                    weight + similar_vehicle.similarity.abs(),
                )
            });
        if denominator != 0.0 {
            Some(target_vehicle.mean_rating + numerator / denominator)
        } else {
            None
        }
    }

    pub fn predict_many<'a>(
        &'a self,
        target_ids: impl IntoIterator<Item = i32> + 'a,
        source_ratings: &'a HashMap<i32, Rating>,
    ) -> impl Iterator<Item = Prediction> + 'a {
        target_ids.into_iter().filter_map(|target_id| {
            self.predict(target_id, source_ratings)
                .map(|rating| Prediction { tank_id: target_id, rating })
        })
    }

    #[must_use]
    fn calculate_biases<'a>(votes: &'a HashMap<i32, Vec<&'a Vote>>) -> Box<[Biased<'a>]> {
        votes
            .par_iter()
            .map(|(tank_id, votes)| {
                let mean_rating = votes.iter().map(|vote| f64::from(vote.rating)).sum::<f64>()
                    / votes.len() as f64;
                Biased {
                    tank_id: *tank_id,
                    mean_rating,
                    votes: votes.as_ref(),
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn calculate_similarities(biased: &[Biased], disable_damping: bool) -> HashMap<i32, Vehicle> {
        biased
            .par_iter()
            .map(|vehicle_i| {
                let similar: Box<[SimilarVehicle]> = biased
                    .iter()
                    .filter(|vehicle_j| vehicle_j.tank_id != vehicle_i.tank_id)
                    .filter_map(|vehicle_j| {
                        // FIXME: I do the same calculation twice: for `(i, j)` and `(j, i)`.
                        Self::calculate_similarity(vehicle_i, vehicle_j, disable_damping).map(
                            |similarity| SimilarVehicle {
                                similarity,
                                tank_id: vehicle_j.tank_id,
                                mean_rating: vehicle_j.mean_rating,
                            },
                        )
                    })
                    .collect();
                let entry = Vehicle {
                    mean_rating: vehicle_i.mean_rating,
                    similar,
                };
                (vehicle_i.tank_id, entry)
            })
            .collect()
    }

    fn calculate_similarity(
        vehicle_i: &Biased,
        vehicle_j: &Biased,
        disable_damping: bool,
    ) -> Option<f64> {
        let (numerator, denominator_i, denominator_j) =
            merge_join_by(vehicle_i.votes, vehicle_j.votes, |i, j| i.account_id.cmp(&j.account_id))
                .fold(
                    (0.0, 0.0, 0.0),
                    |(mut numerator, mut denominator_i, mut denominator_j), either| {
                        match either {
                            EitherOrBoth::Left(vote_i) => {
                                if !disable_damping {
                                    denominator_i +=
                                        (f64::from(vote_i.rating) - vehicle_i.mean_rating).powi(2);
                                }
                            }
                            EitherOrBoth::Right(vote_j) => {
                                if !disable_damping {
                                    denominator_j +=
                                        (f64::from(vote_j.rating) - vehicle_j.mean_rating).powi(2);
                                }
                            }
                            EitherOrBoth::Both(vote_i, vote_j) => {
                                let diff_i = f64::from(vote_i.rating) - vehicle_i.mean_rating;
                                let diff_j = f64::from(vote_j.rating) - vehicle_j.mean_rating;
                                numerator += diff_i * diff_j;
                                denominator_i += diff_i.powi(2);
                                denominator_j += diff_j.powi(2);
                            }
                        }
                        (numerator, denominator_i, denominator_j)
                    },
                );

        if denominator_i != 0.0 && denominator_j != 0.0 {
            Some(numerator / denominator_i.sqrt() / denominator_j.sqrt())
        } else {
            None
        }
    }

    fn sort(vehicles: &mut HashMap<i32, Vehicle>) {
        for entry in vehicles.values_mut() {
            entry
                .similar
                .sort_unstable_by(|lhs, rhs| rhs.similarity.total_cmp(&lhs.similarity))
        }
    }
}

/// Calculated mean rating altogether with the relevant votes.
struct Biased<'a> {
    tank_id: i32,
    mean_rating: f64,
    votes: &'a [&'a Vote],
}

#[derive(Serialize, Deserialize)]
pub struct Vehicle {
    pub mean_rating: f64,

    /// Similar vehicles (tank ID and similarity), sorted by descending similarity.
    pub similar: Box<[SimilarVehicle]>,
}

#[derive(Serialize, Deserialize)]
pub struct SimilarVehicle {
    /// Similar vehicle ID.
    tank_id: i32,

    /// Similarity of this vehicle to the source vehicle.
    similarity: f64,

    /// The similar vehicle's mean rating.
    mean_rating: f64,
}
