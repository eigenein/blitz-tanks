use std::{
    fmt::{Display, Formatter},
    ops::Div,
};

use crate::models::rating::Rating;

/// [(Mean) reciprocal rank][1].
///
/// [1]: https://en.wikipedia.org/wiki/Mean_reciprocal_rank
#[derive(
    derive_more::Into,
    derive_more::Add,
    derive_more::Sum,
    PartialOrd,
    PartialEq,
    Copy,
    Clone,
    Default,
)]
pub struct ReciprocalRank(f64);

impl Display for ReciprocalRank {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.3}", self.0)
    }
}

impl Div<usize> for ReciprocalRank {
    type Output = Self;

    #[inline]
    fn div(self, rhs: usize) -> Self::Output {
        Self(self.0 / rhs as f64)
    }
}

impl FromIterator<Rating> for ReciprocalRank {
    fn from_iter<T: IntoIterator<Item = Rating>>(iter: T) -> Self {
        iter.into_iter()
            .enumerate()
            .find(|(_, rating)| *rating == Rating::Like)
            .map_or(Self(0.0), |(rank, _)| Self(1.0 / ((rank + 1) as f64)))
    }
}
