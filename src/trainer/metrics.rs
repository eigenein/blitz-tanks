use std::{iter::Sum, ops::Div};

use itertools::Itertools;

use crate::{
    models::{rating::Rating, vote::Vote},
    trainer::prediction::Prediction,
};

#[derive(Default)]
pub struct Metrics {
    /// [(Mean) reciprocal rank][1].
    ///
    /// [1]: https://en.wikipedia.org/wiki/Mean_reciprocal_rank
    pub reciprocal_rank: f64,
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
        Self { reciprocal_rank }
    }
}

impl Sum for Metrics {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |sum, item| Metrics {
            reciprocal_rank: sum.reciprocal_rank + item.reciprocal_rank,
        })
    }
}

impl Div<f64> for Metrics {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            reciprocal_rank: self.reciprocal_rank / rhs,
        }
    }
}
