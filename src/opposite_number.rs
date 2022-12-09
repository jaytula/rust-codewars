// https://www.codewars.com/kata/56dec885c54a926dcd001095/solutions/rust

fn opposite(number: i32) -> i32 {
  -number
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn returns_expected() {
  assert_eq!(opposite(1), -1);
  assert_eq!(opposite(-1), 1);
}