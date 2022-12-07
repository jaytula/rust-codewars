// https://www.codewars.com/kata/5aa736a455f906981800360d/solutions/rust

fn feast(beast: &str, dish: &str) -> bool {
    let f1 = beast.chars().nth(0).unwrap();
    let f2 = dish.chars().nth(0).unwrap();
    let e1 = beast.chars().nth(beast.len() - 1).unwrap();
    let e2 = dish.chars().nth(dish.len() - 1).unwrap();

    f1 == f2 && e1 == e2
}

// Rust test example:
#[test]
fn sample_test_cases() {
  assert_eq!(feast("great blue heron", "garlic naan"), true);
  assert_eq!(feast("chickadee", "chocolate cake"), true);
  assert_eq!(feast("brown bear", "bear claw"), false);
}