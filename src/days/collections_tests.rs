#[cfg(test)]
use crate::days::collections;

#[test]
fn test_unique() {
    assert_eq!(collections::unique(&vec![4, 2, 3, 1, 3, 4, 5, 4, 5]), vec![4, 2, 3, 1, 5]);
    assert_eq!(collections::unique(&Vec::<u64>::new()), Vec::<u64>::new());
    assert_eq!(collections::unique(&vec![2, 3, 1]), vec![2, 3, 1]);
}

#[test]
fn test_pairs_of() {
    assert_eq!(collections::pairs_of::<u32>(&vec![]), Vec::new());
    assert_eq!(collections::pairs_of(&vec![1]), Vec::new());
    assert_eq!(collections::pairs_of(&vec![1, 2]), vec![(1, 2)]);
    assert_eq!(collections::pairs_of(&vec![1, 2, 3, 4, 5]), vec![(1, 2), (2, 3), (3, 4), (4, 5)]);
}