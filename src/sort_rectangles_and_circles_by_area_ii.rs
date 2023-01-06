use std::{cmp::Ordering, f64::consts::PI};
use either::Either;

struct MyEither(Either<(f64, f64), f64>);

impl Ord for MyEither {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut num1: f64 =  0.0;
        if self.is_left() {
            let (l, w) = self.left().unwrap();
            num1 = l * w;
        } else {
            let r = self.right().unwrap();
            num1 = PI * r.powf(2);
        }
        let mut num2: f64 = 0.0;
        if other.is_left() {
            let (l, w) = other.left().unwrap();
            num2 = l * w;
        } else {
            let r = other.right().unwrap();
            num2 = PI * r.powf(2);
        }
        if num1 == num2 {
            return Ordering::Equal;
        }
        if num1 < num2 { Ordering::Less } else { Ordering::Greater }
    }
}

fn sort_by_area(_seq: &[Either<(f64, f64), f64>]) -> Vec<Either<(f64, f64), f64>> {
    let mut res = _seq.to_vec();
    res.sort();

    res
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
        assert!(actual == expected, "With seq = {seq:?}\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[Either::Left((4.23, 6.43)), Either::Right(1.23), Either::Right(3.444), Either::Left((1.342, 3.212))], &[Either::Left((1.342, 3.212)), Either::Right(1.23), Either::Left((4.23, 6.43)), Either::Right(3.444)]);
        // dotest(&[Either::Left((2.0, 5.0)), Either::Right(6.0)], &[Either::Left((2.0, 5.0)), Either::Right(6.0) ]);
        // dotest(&[], &[]);
    }
}
