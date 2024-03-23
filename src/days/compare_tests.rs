#[cfg(test)]
use crate::days::compare;
#[cfg(test)]
use std::cmp::Ordering;

#[test]
fn test_compare_arrays_of_equal_length() {
    assert_eq!(compare::compare_arrays(&[2, 3, 4], &[2, 4, 4], |x, y| x.cmp(y)), Ordering::Less);
    assert_eq!(compare::compare_arrays(&[2, 3, 5], &[2, 3, 4], |x, y| x.cmp(y)), Ordering::Greater);
    assert_eq!(compare::compare_arrays(&[2, 3, 4], &[2, 3, 4], |x, y| x.cmp(y)), Ordering::Equal);
    assert_eq!(compare::compare_arrays(&[1, 2, 3], &[2, 3, 4], |x, y| x.cmp(y)), Ordering::Less);
}

#[test]
fn test_compare_arrays_of_different_length() {
    assert_eq!(compare::compare_arrays(&[1, 2, 3], &[1, 2], |x, y| x.cmp(y)), Ordering::Greater); // compared by length
    assert_eq!(compare::compare_arrays(&[1, 2, 3], &[1, 2, 3, 4], |x, y| x.cmp(y)), Ordering::Less); // compared by length
    assert_eq!(compare::compare_arrays(&[1, 2, 3], &[2, 3], |x, y| x.cmp(y)), Ordering::Less); // first element makes first less than second
    assert_eq!(compare::compare_arrays(&[4, 2, 3], &[1, 2, 3, 4, 5], |x, y| x.cmp(y)), Ordering::Greater); // first element makes greater
}