use std::collections::HashMap;

use indicatif::ProgressIterator;
use itertools::{merge_join_by, EitherOrBoth, Itertools};

use crate::{
    models::{rating::Rating, vote::Vote},
    prelude::*,
};

#[derive(Default)]
pub struct FitParams {
    pub disable_damping: bool,
}

pub struct PredictParams {
    pub n_neighbors: u32,
}

#[must_use]
pub struct Model(
    /// Mapping from vehicle's tank ID to other vehicles' similarities.
    ///
    /// # Note
    ///
    /// The mapping values only contain entries,
    /// for which tank ID are **greater** than the respective mapping key.
    HashMap<i32, VehicleEntry>,
);

impl Model {
    pub fn fit(votes: &[Vote], params: &FitParams) -> Self {
        let votes = votes.iter().into_group_map_by(|vote| vote.tank_id);
        let biased = Self::calculate_biases(&votes);
        let mut model = Self::calculate_similarities(&biased, params.disable_damping);
        Self::sort(&mut model);
        Model(model)
    }

    #[must_use]
    pub fn predict(&self, inputs: &HashMap<i32, Rating>, params: &PredictParams) {
        todo!()
    }

    fn calculate_biases<'a>(votes: &'a HashMap<i32, Vec<&'a Vote>>) -> Box<[Biased<'a>]> {
        votes
            .iter()
            .map(|(tank_id, votes)| {
                let mean_rating = votes.iter().map(|vote| f64::from(vote.rating)).sum::<f64>()
                    / votes.len() as f64;
                Biased {
                    tank_id: *tank_id,
                    mean_rating,
                    votes: votes.as_ref(),
                }
            })
            .collect()
    }

    fn calculate_similarities(
        biased: &[Biased],
        disable_damping: bool,
    ) -> HashMap<i32, VehicleEntry> {
        biased
            .iter()
            .progress()
            .map(|vehicle_i| {
                let similar: Box<[(i32, f64)]> = biased
                    .iter()
                    .filter(|vehicle_j| vehicle_j.tank_id > vehicle_i.tank_id)
                    .filter_map(|vehicle_j| {
                        Self::calculate_similarity(vehicle_i, vehicle_j, disable_damping)
                            .map(|similarity| (vehicle_j.tank_id, similarity))
                    })
                    .collect();
                let entry = VehicleEntry {
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

    fn sort(model: &mut HashMap<i32, VehicleEntry>) {
        for entry in model.values_mut() {
            entry.similar.sort_unstable_by(|(_, lhs), (_, rhs)| rhs.total_cmp(lhs))
        }
    }
}

/// Calculated mean rating altogether with the relevant votes.
struct Biased<'a> {
    tank_id: i32,
    mean_rating: f64,
    votes: &'a [&'a Vote],
}

pub struct VehicleEntry {
    pub mean_rating: f64,

    /// Similar vehicles (tank ID and similarity), sorted by descending similarity.
    pub similar: Box<[(i32, f64)]>,
}
