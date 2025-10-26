use std::hash::Hash;

/// A structure that supports probabilistic membership queries.
pub trait ProbabilisticSet {
    /// Insert an item.
    fn insert<T: Hash>(&self, item: &T);

    /// Check if an item is *probably* in the set.
    fn contains<T: Hash>(&self, item: &T) -> bool;
}

/// A structure that supports approximate frequency counting.
pub trait FrequencySketch {
    /// Increment the count for a given item by `count`.
    fn increment<T: Hash>(&self, item: &T, count: u32);

    /// Estimate the frequency of a given item.
    fn frequency<T: Hash>(&self, item: &T) -> u32;
}
