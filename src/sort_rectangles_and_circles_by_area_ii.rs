use either::Either;

fn sort_by_area(seq: &[Either<(f64, f64), f64>]) -> Vec<Either<(f64, f64), f64>> {
    todo!()
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
        dotest(&[Either::Left((2.0, 5.0)), Either::Right(6.0)], &[Either::Left((2.0, 5.0)), Either::Right(6.0) ]);
        dotest(&[], &[]);
    }
}
