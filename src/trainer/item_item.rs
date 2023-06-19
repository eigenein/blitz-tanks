use std::collections::HashMap;

use indicatif::ProgressIterator;
use itertools::{merge_join_by, EitherOrBoth, Itertools};

use crate::{models::vote::Vote, prelude::*, tracing::report_memory_usage};

#[derive(Default)]
pub struct FitParams {
    pub disable_damping: bool,
}

pub struct PredictParams {
    pub n_neighbors: u32,
}

#[must_use]
pub struct Model {
    similarities: HashMap<(i32, i32), f64>,
    // TODO: `biases`.
}

impl Model {
    pub fn fit(votes: &[Vote], params: &FitParams) -> Self {
        let votes = votes.iter().into_group_map_by(|vote| vote.tank_id);
        let biases = Self::calculate_biases(&votes);
        let similarities = Self::calculate_similarities(&votes, &biases, params.disable_damping);
        Model { similarities }
    }

    fn calculate_biases(votes: &HashMap<i32, Vec<&Vote>>) -> Box<[VehicleBias]> {
        info!("⏳ Collecting averages…");
        let biases: Box<[VehicleBias]> = votes
            .iter()
            .map(|(tank_id, votes)| VehicleBias {
                tank_id: *tank_id,
                mean_rating: votes.iter().map(|vote| f64::from(vote.rating)).sum::<f64>()
                    / votes.len() as f64,
            })
            .collect();

        info!(n_vehicles = biases.len(), "✅ Gotcha!");
        report_memory_usage();
        biases
    }

    fn calculate_similarities(
        votes: &HashMap<i32, Vec<&Vote>>,
        biases: &[VehicleBias],
        disable_damping: bool,
    ) -> HashMap<(i32, i32), f64> {
        info!("⏳ Calculating similarities…");
        let entries: HashMap<_, _> = biases
            .iter()
            .progress()
            .cartesian_product(biases.iter())
            .filter(|(vehicle_i, vehicle_j)| {
                // Avoid calculating the same thing twice.
                vehicle_i.tank_id < vehicle_j.tank_id
            })
            .filter_map(|(vehicle_i, vehicle_j)| {
                Self::calculate_similarity(votes, vehicle_i, vehicle_j, disable_damping).map(
                    |similarity| {
                        [
                            // Because we only calculate it an either way, we need to emit the
                            // both entries.
                            ((vehicle_i.tank_id, vehicle_j.tank_id), similarity),
                            ((vehicle_j.tank_id, vehicle_i.tank_id), similarity),
                        ]
                    },
                )
            })
            .flatten()
            .collect();

        info!(n_entries = entries.len(), "✅ Gotcha!");
        report_memory_usage();
        entries
    }

    fn calculate_similarity(
        votes: &HashMap<i32, Vec<&Vote>>,
        vehicle_i: &VehicleBias,
        vehicle_j: &VehicleBias,
        disable_damping: bool,
    ) -> Option<f64> {
        let (numerator, denominator_i, denominator_j) = merge_join_by(
            votes.get(&vehicle_i.tank_id)?,
            votes.get(&vehicle_j.tank_id)?,
            |i, j| i.account_id.cmp(&j.account_id),
        )
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
}

struct VehicleBias {
    pub tank_id: i32,
    pub mean_rating: f64,
}
