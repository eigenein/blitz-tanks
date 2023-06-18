use std::{collections::HashMap, ops::AddAssign};

use futures::{stream, StreamExt, TryFutureExt, TryStreamExt};
use indicatif::ProgressIterator;
use itertools::Itertools;

use crate::{
    db::votes::Votes,
    models::vote::Vote,
    prelude::*,
    trainer::merge_join_by::{merge_join_by, Merge},
};

#[derive(Default)]
pub struct FitParams {
    pub disable_damping: bool,
}

pub struct PredictParams {
    pub n_neighbors: u32,
}

pub struct ModelFitter {
    votes: Votes,
    params: FitParams,
}

impl ModelFitter {
    pub const fn new(votes: Votes, params: FitParams) -> Self {
        Self { votes, params }
    }

    pub async fn fit(&self) -> Result<Model> {
        let biases = self.calculate_biases().await?;
        let similarities = self.calculate_similarities(&biases).await?;
        info!(?similarities);
        unimplemented!()
    }

    async fn calculate_biases(&self) -> Result<Box<[VehicleBias]>> {
        info!("⏳ Collecting averages…");
        let biases: Box<[VehicleBias]> = self
            .votes
            .iter_all()
            .await?
            .try_fold(
                HashMap::<i32, RatingAccumulator>::new(),
                |mut accumulators, vote| async move {
                    *accumulators.entry(vote.tank_id).or_default() += f64::from(vote.rating);
                    Ok(accumulators)
                },
            )
            .await?
            .into_iter()
            .map(|(tank_id, accumulator)| {
                let mean_rating = accumulator.into();
                VehicleBias { tank_id, mean_rating }
            })
            .collect();
        info!(n_vehicles = biases.len(), "✅ Gotcha!");
        Ok(biases)
    }

    async fn calculate_similarities(
        &self,
        biases: &[VehicleBias],
    ) -> Result<HashMap<(i32, i32), f64>> {
        info!("⏳ Calculating similarities…");
        let vehicle_pairs = biases
            .iter()
            .progress()
            .cartesian_product(biases.iter())
            .filter(|(vehicle_i, vehicle_j)| vehicle_i.tank_id < vehicle_j.tank_id);
        stream::iter(vehicle_pairs)
            .map(Ok)
            .try_filter_map(|(vehicle_i, vehicle_j)| async {
                match self.calculate_similarity(vehicle_i, vehicle_j).await? {
                    Some(similarity) => {
                        Ok::<_, Error>(Some(((vehicle_i.tank_id, vehicle_j.tank_id), similarity)))
                    }
                    None => Ok(None),
                }
            })
            .inspect_ok(|((tank_id_i, tank_id_j), similarity)| {
                info!(tank_id_i, tank_id_j, similarity);
            })
            .try_collect()
            .await
            .context("failed to calculate similarities")
    }

    async fn calculate_similarity(
        &self,
        vehicle_i: &VehicleBias,
        vehicle_j: &VehicleBias,
    ) -> Result<Option<f64>> {
        let (numerator, denominator_i, denominator_j) = merge_join_by(
            self.votes.iter_by_tank_id(vehicle_i.tank_id).await?,
            self.votes.iter_by_tank_id(vehicle_j.tank_id).await?,
            |vote| vote.account_id,
        )
        .try_fold(
            (0.0, 0.0, 0.0),
            |(mut numerator, mut denominator_i, mut denominator_j), merge: Merge<Vote>| async move {
                match merge {
                    Merge::Left(vote_i) => {
                        if !self.params.disable_damping {
                            denominator_i +=
                                (f64::from(vote_i.rating) - vehicle_i.mean_rating).powi(2);
                        }
                    }
                    Merge::Right(vote_j) => {
                        if !self.params.disable_damping {
                            denominator_j +=
                                (f64::from(vote_j.rating) - vehicle_j.mean_rating).powi(2);
                        }
                    }
                    Merge::Both(vote_i, vote_j) => {
                        let diff_i = f64::from(vote_i.rating) - vehicle_i.mean_rating;
                        let diff_j = f64::from(vote_j.rating) - vehicle_j.mean_rating;
                        numerator += diff_i * diff_j;
                        denominator_i += diff_i.powi(2);
                        denominator_j += diff_j.powi(2);
                    }
                }
                Ok((numerator, denominator_i, denominator_j))
            },
        )
        .await?;

        if denominator_i != 0.0 && denominator_j != 0.0 {
            Ok(Some(numerator / denominator_i.sqrt() / denominator_j.sqrt()))
        } else {
            Ok(None)
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
