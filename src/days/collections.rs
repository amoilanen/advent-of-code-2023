pub fn unique<T: Eq + Clone>(vec: &Vec<T>) -> Vec<T>
{
  let mut result: Vec<T> = Vec::new();
  for elem in vec.iter() {
      if !result.contains(elem) {
          result.push(elem.clone());
      }
  }
  result
}