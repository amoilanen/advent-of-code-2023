pub fn unique<T: Eq + Clone>(vec: &Vec<T>) -> Vec<T> {
  let mut result: Vec<T> = Vec::new();
  for elem in vec.iter() {
      if !result.contains(elem) {
          result.push(elem.clone());
      }
  }
  result
}

pub fn pairs_of<T: Clone>(values: &Vec<T>) -> Vec<(T, T)> {
    values.iter()
      .zip(values.iter().skip(1))
      .map(|(cur, next)|
         (cur.clone(), next.clone())
       )
       .collect()
}