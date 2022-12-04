// https://www.codewars.com/kata/5a3fe3dde1ce0e8ed6000097/solutions/rust

fn century(year: u32) -> u32 {
    year / 100 + (if year % 100 > 0 { 1 } else { 0 })
}

#[test]
fn basic_tests() {
  assert_eq!(century(1705), 18);
  assert_eq!(century(1900), 19);
  assert_eq!(century(1601), 17);
  assert_eq!(century(2000), 20);
  assert_eq!(century(89), 1);
}