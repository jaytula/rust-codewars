// https://www.codewars.com/kata/53d32bea2f2a21f666000256/solutions/rust

fn largest(n: usize, xs: &[i32]) -> Vec<i32> {
    let mut vec = xs.to_vec();
    vec.sort();
    let res = &vec[(vec.len()-n)..];
    res.to_vec()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::largest;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(n: usize, xs: &[i32], expected: Vec<i32>) {
        assert_eq!(
            largest(n, xs),
            expected,
            "{ERR_MSG} with n = {n}, xs = {xs:?}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(2, &[10, 9, 8, 7, 6, 5, 4, 3, 2, 1], vec![9, 10]);
    }
}
