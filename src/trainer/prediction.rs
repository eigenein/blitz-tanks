use std::cmp::Ordering;

/// Model prediction.
///
/// # Ordering
///
/// Prediction with a **higher** rating comes **before** prediction with a **lower** rating.
#[derive(Debug)]
pub struct Prediction {
    pub tank_id: u16,
    pub rating: f64,
}

impl PartialEq for Prediction {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.rating == other.rating
    }
}

impl Eq for Prediction {}

impl PartialOrd<Self> for Prediction {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.rating.partial_cmp(&self.rating)
    }
}

impl Ord for Prediction {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        other.rating.total_cmp(&self.rating)
    }
}
