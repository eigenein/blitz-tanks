use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Add, Div},
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
    Debug,
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
            .map_or(Self::default(), |(rank, _)| ReciprocalRank(1.0 / ((rank + 1) as f64)))
    }
}

pub struct Mean<T>(pub T);

impl<T> FromIterator<T> for Mean<T>
where
    T: Default,
    T: Add<Output = T>,
    T: Div<usize, Output = T>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let (n, sum) = iter
            .into_iter()
            .fold((0_usize, T::default()), |(n, sum), next| (n + 1, sum + next));
        Self(if n != 0 { sum / n } else { T::default() })
    }
}
