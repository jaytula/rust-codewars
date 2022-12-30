// https://www.codewars.com/kata/51f2d1cafc9c0f745c00037d/solutions/rust

fn solution(word: &str, ending: &str) -> bool {
    if ending.len() > word.len() { return false; }
    &word[(word.len()-ending.len())..] == ending
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn returns_expected() {
  assert_eq!(true, solution("abc", "c"));
  assert_eq!(false, solution("strawberry", "banana"));
}