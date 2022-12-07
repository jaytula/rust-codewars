// https://www.codewars.com/kata/5a805d8cafa10f8b930005ba/solutions/rust

fn nearest_sq(n: u32) -> u32 {
    let root1 = (n as f32).sqrt().floor() as u32;
    let root2 = root1 + 1;

    let sq1 = root1.pow(2);
    let sq2 = root2.pow(2);

    if n - sq1 <= sq2 - n {
        return sq1;
    }
    sq2
}

#[cfg(test)]
mod tests {
    use super::nearest_sq;

    #[test]
    fn sample_tests() {
        // assertion(expected, n)
        assertion(1, 1);
        assertion(1, 2);
        assertion(9, 10);
        assertion(121, 111);
        assertion(10000, 9999);
    }

    fn assertion(expected: u32, n: u32) {
        let actual = nearest_sq(n);
        assert!(
            expected == actual,
            "\nTest failed!\n expected: {}\n actual: {}\n n: {}\n",
            expected,
            actual,
            n
        );
    }
}
