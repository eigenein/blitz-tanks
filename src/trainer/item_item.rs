use std::{collections::HashMap, ops::AddAssign};

use futures::{stream, StreamExt, TryStreamExt};
use itertools::Itertools;

use crate::{db::votes::Votes, prelude::*};

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
        let vehicle_pairs = biases
            .iter()
            .cartesian_product(biases.iter())
            .filter(|(vehicle_i, vehicle_j)| vehicle_i.tank_id < vehicle_j.tank_id);
        stream::iter(vehicle_pairs)
            .then(|(vehicle_i, vehicle_j)| async {
                let similarity = self.calculate_similarity(vehicle_i, vehicle_j).await?;
                Ok::<_, Error>(((vehicle_i.tank_id, vehicle_j.tank_id), similarity))
            })
            .try_collect()
            .await
            .context("failed to calculate similarities")
    }

    async fn calculate_similarity(
        &self,
        vehicle_i: &VehicleBias,
        vehicle_j: &VehicleBias,
    ) -> Result<f64> {
        let mut votes_i = self.votes.iter_by_tank_id(vehicle_i.tank_id).await?;
        let mut votes_j = self.votes.iter_by_tank_id(vehicle_j.tank_id).await?;

        let mut numerator = 0.0;
        let mut denominator_i = 0.0_f64;
        let mut denominator_j = 0.0_f64;

        Ok(numerator / denominator_i.sqrt() / denominator_j.sqrt())
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
