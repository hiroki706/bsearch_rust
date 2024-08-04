//! binary search with using range syntax.

use super::slice_search::SliceSearch;
use std::ops::RangeBounds;
pub trait Range<T, R> {
    fn range(&self, range: R) -> &[T];
    fn range_mut(&mut self, range: R) -> &mut [T];
}

impl<T, R> Range<T, R> for [T]
where
    T: Ord + Copy,
    R: RangeBounds<T>,
{
    /// Get a slice of the array that satisfies the range.
    /// # Arguments
    /// * `range` - The range to search.
    /// # Examples
    /// ```
    /// use bsearch::Range;
    /// let arr = [1, 3, 4, 4, 4, 5, 5, 7, 9];
    ///
    /// assert_eq!(arr.range(4..=4).len(), 3);
    /// assert_eq!(arr.range(5..9), [5, 5, 7]);
    ///
    /// assert_eq!(arr.range(4..4), []);
    /// ```
    fn range(&self, range: R) -> &[T] {
        let n = self.len();
        let start = match range.start_bound() {
            std::ops::Bound::Included(&s) => self.lower_bound(s).unwrap_or(0),
            std::ops::Bound::Excluded(&s) => self.upper_bound(s).unwrap_or(0),
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&e) => self.upper_bound(e).unwrap_or(n),
            std::ops::Bound::Excluded(&e) => self.lower_bound(e).unwrap_or(n),
            std::ops::Bound::Unbounded => self.len(),
        };
        &self[start..end]
    }
    fn range_mut(&mut self, range: R) -> &mut [T] {
        let n = self.len();
        let start = match range.start_bound() {
            std::ops::Bound::Included(&s) => self.lower_bound(s).unwrap_or(0),
            std::ops::Bound::Excluded(&s) => self.upper_bound(s).unwrap_or(0),
            std::ops::Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            std::ops::Bound::Included(&e) => self.upper_bound(e).unwrap_or(n),
            std::ops::Bound::Excluded(&e) => self.lower_bound(e).unwrap_or(n),
            std::ops::Bound::Unbounded => self.len(),
        };
        &mut self[start..end]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_range() {
        let arr = [1, 3, 4, 4, 4, 5, 5, 7, 9];
        assert_eq!(arr.range(3..6), [3, 4, 4, 4, 5, 5]);
        assert_eq!(arr.range(3..=4), [3, 4, 4, 4]);
        assert_eq!(arr.range(..4), [1, 3]);
        assert_eq!(arr.range(4..), [4, 4, 4, 5, 5, 7, 9]);
        assert_eq!(arr.range(..), [1, 3, 4, 4, 4, 5, 5, 7, 9]);
        assert_eq!(arr.range(4..4), []);
        // count 4 in arr
        assert_eq!(arr.range(4..=4).len(), 3);
        // count 3 <= x <= 5 in arr
        assert_eq!(arr.range(3..=5).len(), 6);
    }
    #[test]
    fn test_range_mut() {
        let mut arr = [1, 3, 4, 4, 4, 5, 5, 7, 9];
        let slice = arr.range_mut(4..=4);
        slice[0] = 0;
        assert_eq!(slice, [0, 4, 4]);
        assert_eq!(arr, [1, 3, 0, 4, 4, 5, 5, 7, 9]);
    }
}
