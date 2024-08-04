//! This crate provides binary search algorithms.
pub mod find_bound;
pub mod slice_range;
pub mod slice_search;

pub use find_bound::*;
pub use slice_range::Range;
pub use slice_search::SliceSearch;
