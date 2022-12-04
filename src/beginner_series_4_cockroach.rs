// https://www.codewars.com/kata/55fab1ffda3e2e44f00000c6/solutions/rust

fn cockroach_speed( s : f64 ) -> i64 {
    (s / 3600.0 * 100000.0).floor() as i64
}

#[test]
fn basic_tests() {
  assert_eq!(cockroach_speed(1.08), 30);
  assert_eq!(cockroach_speed(1.09), 30);
  assert_eq!(cockroach_speed(0.0), 0);
}