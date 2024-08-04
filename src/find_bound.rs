//! core functions for binary search.
use std::ops::{Add, Div, Range, Sub};

/// Find the minimum value that satisfies the condition of function `f`.
/// # Arguments
/// * `range` - The search range. start is inclusive, end is exclusive.
/// * `f` - The condition function. f must be a monotonically increasing.
/// [Monotonic_function](https://en.wikipedia.org/wiki/Monotonic_function)
/// # Errors
/// Returns `Err` with the last element of the range if no value in the range satisfies the condition.
/// # Examples
/// ```
/// use bsearch::find_min_match;
///
/// let arr = [false, false, true, true, true];
/// assert_eq!(find_min_match(0..arr.len(), |&x| arr[x]), Ok(2));
///
/// let f = |&x: &i32| 3*x >= 10;
/// assert_eq!(find_min_match(-100..100i32, f), Ok(4));
///
/// assert_eq!(find_min_match(-100..100i32, |_|false), Err(99));
/// ```
pub fn find_min_match<F, T>(range: Range<T>, f: F) -> Result<T, T>
where
    T: Ord + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Copy + From<u8>,
    F: Fn(&T) -> bool,
{
    let (one, two) = (1.into(), 2.into());
    let (mut left, mut right) = (range.start, range.end - one);
    while left < right {
        let mid = (right + left) / two;
        if f(&mid) {
            right = mid;
        } else {
            left = mid + one;
        }
    }

    if f(&right) {
        Ok(right)
    } else {
        Err(right)
    }
}

/// Find the maximum value that satisfies the condition of function `f`.
/// # Arguments
/// * `range` - The search range. start is inclusive, end is exclusive.
/// * `f` - The condition function. f must be a monotonically decreasing.
/// [Monotonic_function](https://en.wikipedia.org/wiki/Monotonic_function)
/// # Errors
/// Returns `Err` with the first element of the range if no value in the range satisfies the condition.
/// # Examples
/// ```
/// use bsearch::find_max_match;
/// let arr = [true, true, false, false, false];
/// assert_eq!(find_max_match(0..arr.len(), |&x| arr[x]), Ok(1));
///
/// let f = |&x: &i32| 3*x <= 10;
/// assert_eq!(find_max_match(-100..100i32, f), Ok(3));
///
/// assert_eq!(find_max_match(-100..100i32, |_|false), Err(-100));
/// ```
pub fn find_max_match<F, T>(range: Range<T>, f: F) -> Result<T, T>
where
    T: Ord + Add<Output = T> + Sub<Output = T> + Div<Output = T> + From<u8> + Copy,
    F: Fn(&T) -> bool,
{
    let (one, two) = (1.into(), 2.into());
    let (mut left, mut right) = (range.start, range.end - one);
    while left < right {
        let mid = (right + left + one) / two;
        if f(&mid) {
            left = mid;
        } else {
            right = mid - one;
        }
    }
    if f(&left) {
        Ok(left)
    } else {
        Err(left)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_min_match() {
        let arr = [false, false, true, true, true];
        assert_eq!(find_min_match(0..arr.len(), |&x| arr[x]), Ok(2));

        let f = |&x: &i32| 3 * x >= 10;
        assert_eq!(find_min_match(-100..100i32, f), Ok(4));
    }

    #[test]
    fn test_find_min_match_err() {
        let arr = [false, false, false, false, false];
        assert_eq!(find_min_match(0..arr.len(), |&x| arr[x]), Err(4));

        let f = |&_: &i32| false;
        assert_eq!(find_min_match(-100..100i32, f), Err(99));
    }

    #[test]
    fn test_find_max_match() {
        let arr = [true, true, false, false, false];
        assert_eq!(find_max_match(0..arr.len(), |&x| arr[x]), Ok(1));

        let arr = [true, false];
        assert_eq!(find_max_match(0..arr.len(), |&x| arr[x]), Ok(0));

        let f = |&x: &i32| 3 * x <= 10;
        assert_eq!(find_max_match(-100..100i32, f), Ok(3));
    }

    #[test]
    fn test_find_max_match_err() {
        let arr = [false, false, false, false, false];
        assert_eq!(find_max_match(0..arr.len(), |&x| arr[x]), Err(0));

        let f = |&x: &i32| x <= -101;
        assert_eq!(find_max_match(-100..100i32, f), Err(-100));
    }
}
