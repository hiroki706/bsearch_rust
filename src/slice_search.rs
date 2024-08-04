//! probides `lower_bound` and `upper_bound` for slices.

use super::find_bound::find_min_match;

pub trait SliceSearch<T> {
    fn lower_bound(&self, value: T) -> Result<usize, usize>;
    fn upper_bound(&self, value: T) -> Result<usize, usize>;
}

impl<T> SliceSearch<T> for [T]
where
    T: Ord,
{
    /// Find the first element in the array that is greater than or equal to the given value.
    /// # Arguments
    /// * `arr` - The array to search. arr must be sorted.
    /// * `value` - The value to search for.
    /// # Errors
    /// Returns `Err` with the last index of the array if no value in the array greater than or equal to the given value.
    /// # Examples
    /// ```
    /// use bsearch::SliceSearch;
    /// let arr = [1, 3, 4, 4, 4, 5, 7, 9];
    /// assert_eq!(arr.lower_bound(1), Ok(0));
    /// assert_eq!(arr.lower_bound(4), Ok(2));
    ///
    /// assert_eq!(arr.lower_bound(10), Err(7));
    /// ```
    fn lower_bound(&self, value: T) -> Result<usize, usize> {
        find_min_match(0..self.len(), |&x| self[x] >= value)
    }

    /// Find the first element in the array that is greater than the given value.
    /// # Arguments
    /// * `arr` - The array to search. arr must be sorted.
    /// * `value` - The value to search for.
    /// # Errors
    /// Returns `Err` with the last index of the array if no value in the array greater than the given value.
    /// # Examples
    /// ```
    /// use bsearch::SliceSearch;
    /// let arr = [1, 3, 4, 4, 4, 5, 7, 9];
    /// assert_eq!(arr.upper_bound(1), Ok(1));
    /// assert_eq!(arr.upper_bound(4), Ok(5));
    ///
    /// assert_eq!(arr.upper_bound(10), Err(7));
    /// ```
    fn upper_bound(&self, value: T) -> Result<usize, usize> {
        find_min_match(0..self.len(), |&x| self[x] > value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_lower_bound_ok() {
        let arr = [1, 3, 4, 4, 4, 5, 7, 9];
        assert_eq!(arr.lower_bound(1), Ok(0));
        assert_eq!(arr.lower_bound(4), Ok(2));
        assert_eq!(arr.lower_bound(6), Ok(6));
    }
    #[test]
    fn test_lower_bound_err() {
        let arr = [1, 3, 4, 4, 4, 5, 7, 9];
        assert_eq!(arr.lower_bound(10), Err(7));
        let tf = [false, false, false, false, false];
        assert_eq!(tf.lower_bound(true), Err(4));
    }
    #[test]
    fn test_upper_bound() {
        let arr = [1, 3, 4, 4, 4, 5, 7, 9];
        assert_eq!(arr.upper_bound(1), Ok(1));
        assert_eq!(arr.upper_bound(4), Ok(5));
        assert_eq!(arr.upper_bound(6), Ok(6));
    }
    #[test]
    fn test_upper_bound_err() {
        let arr = [1, 3, 4, 4, 4, 5, 7, 9];
        assert_eq!(arr.upper_bound(10), Err(7));
        let tf = [false, false, false, false, false];
        assert_eq!(tf.upper_bound(true), Err(4));
    }
}
