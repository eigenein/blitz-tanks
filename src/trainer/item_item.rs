use std::collections::HashMap;

use itertools::{merge_join_by, EitherOrBoth, Itertools};
use rayon::prelude::*;

use crate::{
    models::{rating::Rating, vote::Vote},
    prelude::*,
    trainer::prediction::Prediction,
};

#[derive(Default, Debug)]
pub struct FitParams {
    pub disable_damping: bool,
}

#[derive(Debug)]
pub struct PredictParams {
    pub n_neighbors: usize,

    /// Include negative similarities.
    pub include_negative: bool,
}

/// Item-item kNN collaborative filtering.
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
    #[instrument(skip_all, fields(for_tank_id = for_tank_id))]
    pub fn predict(
        &self,
        for_tank_id: i32,
        from: &HashMap<i32, Rating>,
        params: &PredictParams,
    ) -> Option<f64> {
        let target_vehicle = self.0.get(&for_tank_id)?;
        let (numerator, denominator, n_neighbors) = target_vehicle
            .similar
            .iter()
            .filter(|(_, similarity)| params.include_negative || (*similarity > 0.0))
            .filter_map(|(similar_tank_id, similarity)| {
                from.get(similar_tank_id)
                    .map(|rating| (*similar_tank_id, *similarity, f64::from(*rating)))
            })
            .take(params.n_neighbors)
            .fold(
                (0.0, 0.0, 0_usize),
                |(numerator, denominator, n_neighbors), (similar_tank_id, similarity, rating_j)| {
                    (
                        numerator
                            + similarity
                                * (rating_j - self.0.get(&similar_tank_id).unwrap().mean_rating),
                        denominator + similarity.abs(),
                        n_neighbors + 1,
                    )
                },
            );
        if denominator != 0.0 {
            trace!(n_neighbors);
            Some(target_vehicle.mean_rating + numerator / denominator)
        } else {
            None
        }
    }

    pub fn predict_many<'a>(
        &'a self,
        for_tank_ids: impl IntoIterator<Item = i32> + 'a,
        from: &'a HashMap<i32, Rating>,
        params: &'a PredictParams,
    ) -> impl Iterator<Item = Prediction> + 'a {
        for_tank_ids.into_iter().filter_map(|tank_id| {
            self.predict(tank_id, from, params).map(|rating| Prediction { tank_id, rating })
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

    fn calculate_similarities(
        biased: &[Biased],
        disable_damping: bool,
    ) -> HashMap<i32, VehicleEntry> {
        biased
            .par_iter()
            .map(|vehicle_i| {
                let similar: Box<[(i32, f64)]> = biased
                    .iter()
                    .filter(|vehicle_j| vehicle_j.tank_id != vehicle_i.tank_id)
                    .filter_map(|vehicle_j| {
                        // FIXME: I do the same calculation twice: for `(i, j)` and `(j, i)`.
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
