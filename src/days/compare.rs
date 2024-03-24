use std::cmp::Ordering;

pub fn compare_arrays<T, F>(first: &[T], second: &[T], compare_by: F) -> Ordering
where
  F: Fn(&T, &T) -> Ordering
{
  for (a, b) in first.iter().zip(second.iter()) {
    match compare_by(a, b) {
      Ordering::Less => return Ordering::Less,
      Ordering::Greater => return Ordering::Greater,
      Ordering::Equal => continue,
    }
  }

  if first.len() == second.len() {
    Ordering::Equal
  } else if first.len() < second.len() {
    Ordering::Less
  } else {
    Ordering::Greater
  }
}