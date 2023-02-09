// https://www.codewars.com/kata/5ac6932b2f317b96980000ca/solutions/rust

use itertools::Itertools;

fn min_value(mut digits: Vec<i32>) -> i32 {
    digits.sort();
    let mut last = -1;
    let mut res = Vec::new();

    for digit in digits.into_iter() {
      if digit == last { continue; }
      last = digit;
      res.push(digit);
    }

    res.iter().map(|&el| el.to_string()).join("").parse::<i32>().unwrap_or_default()
}

#[test]
fn basic_test() {
  assert_eq!(min_value(vec![1, 3, 1]), 13);
  assert_eq!(min_value(vec![4, 7, 5, 7]), 457);
  assert_eq!(min_value(vec![4, 8, 1, 4]), 148);
}