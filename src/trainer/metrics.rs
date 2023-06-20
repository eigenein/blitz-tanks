use std::{iter::Sum, ops::Div};

use itertools::Itertools;

use crate::{
    models::{rating::Rating, vote::Vote},
    trainer::prediction::Prediction,
};

#[derive(Default, Debug)]
pub struct Metrics {
    /// [(Mean) reciprocal rank][1].
    ///
    /// [1]: https://en.wikipedia.org/wiki/Mean_reciprocal_rank
    pub reciprocal_rank: f64,

    pub mean_absolute_error: f64,

    pub root_mean_squared_error: f64,
}

impl<'a, I> From<I> for Metrics
where
    I: IntoIterator<Item = (Prediction, &'a Vote)>,
{
    fn from(predictions: I) -> Self {
        let predictions: Box<[(Prediction, &'a Vote)]> = predictions
            .into_iter()
            .sorted_unstable_by(|lhs, rhs| lhs.0.cmp(&rhs.0))
            .collect();

        let reciprocal_rank = predictions
            .iter()
            .enumerate()
            .find(|(_, (_, vote))| vote.rating == Rating::Like)
            .map_or(0.0, |(rank, _)| 1.0 / (rank + 1) as f64);

        let mean_absolute_error = if !predictions.is_empty() {
            predictions
                .iter()
                .map(|(prediction, vote)| (prediction.rating - f64::from(vote.rating)).abs())
                .sum::<f64>()
                / predictions.len() as f64
        } else {
            0.0
        };

        let mean_squared_error = if !predictions.is_empty() {
            predictions
                .iter()
                .map(|(prediction, vote)| (prediction.rating - f64::from(vote.rating)).powi(2))
                .sum::<f64>()
                / predictions.len() as f64
        } else {
            0.0
        };

        Self {
            reciprocal_rank,
            mean_absolute_error,
            root_mean_squared_error: mean_squared_error.sqrt(),
        }
    }
}

impl Sum for Metrics {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |sum, item| Metrics {
            reciprocal_rank: sum.reciprocal_rank + item.reciprocal_rank,
            mean_absolute_error: sum.mean_absolute_error + item.mean_absolute_error,
            root_mean_squared_error: sum.root_mean_squared_error + item.root_mean_squared_error,
        })
    }
}

impl Div<f64> for Metrics {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            reciprocal_rank: self.reciprocal_rank / rhs,
            mean_absolute_error: self.mean_absolute_error / rhs,
            root_mean_squared_error: self.root_mean_squared_error / rhs,
        }
    }
}
