use std::{collections::HashMap, ops::AddAssign};

use indicatif::ProgressIterator;
use itertools::{merge_join_by, EitherOrBoth, Itertools};

use crate::{models::vote::Vote, prelude::*};

#[derive(Default)]
pub struct FitParams {
    pub disable_damping: bool,
}

pub struct PredictParams {
    pub n_neighbors: u32,
}

pub struct ModelFitter<'a> {
    votes: &'a [Vote],
    params: FitParams,
}

impl<'a> ModelFitter<'a> {
    pub const fn new(votes: &'a [Vote], params: FitParams) -> Self {
        Self { votes, params }
    }

    pub fn fit(&self) -> Model {
        let biases = self.calculate_biases();
        let similarities = self.calculate_similarities(&biases);
        unimplemented!()
    }

    fn calculate_biases(&self) -> Box<[VehicleBias]> {
        info!("⏳ Collecting averages…");
        let biases: Box<[VehicleBias]> = self
            .votes
            .iter()
            .fold(HashMap::<i32, RatingAccumulator>::new(), |mut accumulators, vote| {
                *accumulators.entry(vote.tank_id).or_default() += f64::from(vote.rating);
                accumulators
            })
            .into_iter()
            .map(|(tank_id, accumulator)| {
                let mean_rating = accumulator.into();
                VehicleBias { tank_id, mean_rating }
            })
            .collect();
        info!(n_vehicles = biases.len(), "✅ Gotcha!");
        biases
    }

    fn calculate_similarities(&self, biases: &[VehicleBias]) -> HashMap<(i32, i32), f64> {
        info!("⏳ Calculating similarities…");
        biases
            .iter()
            .progress()
            .cartesian_product(biases.iter())
            .filter(|(vehicle_i, vehicle_j)| vehicle_i.tank_id < vehicle_j.tank_id)
            .filter_map(|(vehicle_i, vehicle_j)| {
                self.calculate_similarity(vehicle_i, vehicle_j).map(|similarity| {
                    [
                        ((vehicle_i.tank_id, vehicle_j.tank_id), similarity),
                        ((vehicle_j.tank_id, vehicle_i.tank_id), similarity),
                    ]
                })
            })
            .flatten()
            .collect()
    }

    fn calculate_similarity(
        &self,
        vehicle_i: &VehicleBias,
        vehicle_j: &VehicleBias,
    ) -> Option<f64> {
        let (numerator, denominator_i, denominator_j) = merge_join_by(
            self.votes.iter().filter(|vote| vote.tank_id == vehicle_i.tank_id),
            self.votes.iter().filter(|vote| vote.tank_id == vehicle_j.tank_id),
            |i, j| i.account_id.cmp(&j.account_id),
        )
        .fold(
            (0.0, 0.0, 0.0),
            |(mut numerator, mut denominator_i, mut denominator_j), either| {
                match either {
                    EitherOrBoth::Left(vote_i) => {
                        if !self.params.disable_damping {
                            denominator_i +=
                                (f64::from(vote_i.rating) - vehicle_i.mean_rating).powi(2);
                        }
                    }
                    EitherOrBoth::Right(vote_j) => {
                        if !self.params.disable_damping {
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

pub struct Model {}

impl Model {}

/// Sum of ratings and number of them.
///
/// It's used to calculate mean vehicle ratings.
#[derive(Default)]
struct RatingAccumulator {
    sum: f64,
    n: u32,
}

impl AddAssign<f64> for RatingAccumulator {
    /// Accumulate the rating.
    #[inline]
    fn add_assign(&mut self, rating: f64) {
        self.sum += rating;
        self.n += 1;
    }
}

impl From<RatingAccumulator> for f64 {
    #[inline]
    fn from(sum: RatingAccumulator) -> Self {
        sum.sum / sum.n as f64
    }
}

struct VehicleBias {
    pub tank_id: i32,
    pub mean_rating: f64,
}
