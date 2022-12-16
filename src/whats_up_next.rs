// https://www.codewars.com/kata/542ebbdb494db239f8000046/solutions/rust

fn next_item<T: PartialEq<T> + Clone>(slice: &[T], find: T) -> Option<T> {
    let mut found = false;
    for item in slice.iter() {
      let cloned_item = item.clone();
      if found { return Some(cloned_item); }
      if cloned_item == find {
        found = true;
      }
    }
    None
}

#[test]
fn returns_expected() {
  assert_eq!(next_item(&["Joe", "Bob", "Sally"], "Bob"), Some("Sally"));
  assert_eq!(next_item(&[0, 1], 0), Some(1));
  assert_eq!(next_item(&[0, 1], 1), None);
  assert_eq!(next_item((1..10).collect::<Vec<_>>().as_slice(), 7), Some(8));  
}