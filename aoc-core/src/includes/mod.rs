pub mod includes_range;

/// Describes a thing that can be thought to "include" (or "contain") another thing.
pub trait Includes<T> {
    /// Returns whether or not `self` fully contains `other`.
    fn includes(&self, other: &T) -> bool;
}
