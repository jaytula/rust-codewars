// https://www.codewars.com/kata/5ce04eadd103f4001edd8986/solutions/rust

fn solution(n: u8, b: u32) -> Vec<u32> {
    if b == 0 { return [].to_vec(); }
    (1..2u32.pow(n as u32)).filter(|num| num & b == b).collect()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::solution;
    
    fn dotest(n: u8, b: u32, expected: &[u32]) {
        let actual = solution(n, b);
        assert!(actual == expected, "With n = {n}, b = {b}\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(4, 2, &[2, 3, 6, 7, 10, 11, 14, 15]);
        dotest(6, 8, &[8, 9, 10, 11, 12, 13, 14, 15, 24, 25, 26, 27, 28, 29, 30, 31, 40, 41, 42, 43, 44, 45, 46, 47, 56, 57, 58, 59, 60, 61, 62, 63]);
        dotest(5, 32, &[]);
        dotest(6, 0, &[]);
        dotest(0, 1, &[]);
    }
}