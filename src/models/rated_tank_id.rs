use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

/// Pair of tank ID and its rating.
///
/// # Notes
///
/// - Natural sorting order is of decreasing rating.
/// - I also use it for tank ID & similarity pair.
#[derive(Serialize, Deserialize)]
pub struct RatedTankId(pub u16, pub f64);

impl Eq for RatedTankId {}

impl PartialEq<Self> for RatedTankId {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        other.1.eq(&other.1)
    }
}

impl PartialOrd<Self> for RatedTankId {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.1.partial_cmp(&self.1)
    }
}

impl Ord for RatedTankId {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.total_cmp(&self.1)
    }
}

impl RatedTankId {
    #[inline]
    pub fn is_positive(&self) -> bool {
        self.1 > 0.0
    }
}
