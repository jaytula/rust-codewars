// https://www.codewars.com/kata/53da3dbb4a5168369a0000fe/solutions/rust

fn even_or_odd(i: i32) -> &'static str {
    if i % 2 == 0 { return "Even"; }
    "Odd"
}

#[test]
fn returns_expected() {
  assert_eq!(even_or_odd(0), "Even");
  assert_eq!(even_or_odd(2), "Even");
  assert_eq!(even_or_odd(1), "Odd");
  assert_eq!(even_or_odd(7), "Odd");
  assert_eq!(even_or_odd(-1), "Odd");
}