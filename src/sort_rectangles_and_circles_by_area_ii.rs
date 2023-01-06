// https://www.codewars.com/kata/5a1ebc2480171f29cf0000e5/solutions/rust

use either::Either;
use itertools::Itertools;
use std::{cmp::Ordering, f64::consts::PI};

struct MyEither(Either<(f64, f64), f64>);

impl Ord for MyEither {
    fn cmp(&self, other: &Self) -> Ordering {
        let num1 = if self.0.is_left() {
            let (l, w) = self.0.left().unwrap();
            l * w
        } else {
            let r = self.0.right().unwrap();
            PI * r.powf(2.0)
        };
        let num2 = if other.0.is_left() {
            let (l, w) = other.0.left().unwrap();
            l * w
        } else {
            let r = other.0.right().unwrap();
            PI * r.powf(2.0)
        };
        if num1 == num2 {
            return Ordering::Equal;
        }
        if num1 < num2 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for MyEither {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let num1 = if self.0.is_left() {
            let (l, w) = self.0.left().unwrap();
            l * w
        } else {
            let r = self.0.right().unwrap();
            PI * r.powf(2.0)
        };
        let num2 = if other.0.is_left() {
            let (l, w) = other.0.left().unwrap();
            l * w
        } else {
            let r = other.0.right().unwrap();
            PI * r.powf(2.0)
        };
        if num1 == num2 {
            return Some(Ordering::Equal);
        }
        if num1 < num2 {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl PartialEq for MyEither {
    fn eq(&self, other: &Self) -> bool {
        let num1 = if self.0.is_left() {
            let (l, w) = self.0.left().unwrap();
            l * w
        } else {
            let r = self.0.right().unwrap();
            PI * r.powf(2.0)
        };
        let num2 = if other.0.is_left() {
            let (l, w) = other.0.left().unwrap();
            l * w
        } else {
            let r = other.0.right().unwrap();
            PI * r.powf(2.0)
        };
        num1 == num2
    }
}
impl Eq for MyEither {}

fn sort_by_area(_seq: &[Either<(f64, f64), f64>]) -> Vec<Either<(f64, f64), f64>> {
    let mut res = _seq
        .to_vec()
        .iter()
        .map(|el| MyEither(*el))
        .collect::<Vec<_>>();
    res.sort();

    res.iter().map(|el| el.0).collect_vec()
    // [Either::Left((1.342 as f64, 3.212 as f64)), Either::Right(1.23), Either::Left((4.23, 6.43)), Either::Right(3.444)].to_vec()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::sort_by_area;
    use either::Either;

    fn dotest(seq: &[Either<(f64, f64), f64>], expected: &[Either<(f64, f64), f64>]) {
        let actual = sort_by_area(seq);
        assert!(
            actual == expected,
            "With seq = {seq:?}\nExpected {expected:?} but got {actual:?}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(
            &[
                Either::Left((4.23, 6.43)),
                Either::Right(1.23),
                Either::Right(3.444),
                Either::Left((1.342, 3.212)),
            ],
            &[
                Either::Left((1.342, 3.212)),
                Either::Right(1.23),
                Either::Left((4.23, 6.43)),
                Either::Right(3.444),
            ],
        );
        dotest(&[Either::Left((2.0, 5.0)), Either::Right(6.0)], &[Either::Left((2.0, 5.0)), Either::Right(6.0) ]);
        dotest(&[], &[]);
    }
}
