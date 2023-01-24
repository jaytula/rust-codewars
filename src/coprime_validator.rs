// https://www.codewars.com/kata/585c50e75d0930e6a7000336/solutions/rust

fn are_coprime(n: u8, m: u8) -> bool {
    let high = n.max(m);
    let square_root = (high as f32 / 2.0).ceil() as u8;
    
    if n == m { return false; }
    for f in 2u8..=square_root {
        if n % f == 0 && m % f == 0 { return false; }
    }
    true
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::are_coprime;
    
    fn dotest(n: u8, m: u8, expected: bool) {
        let actual = are_coprime(n, m);
        assert!(actual == expected, "Test failed with n = {n} and m = {m}\nExpected {expected} but got {actual}")
    } 
    
    #[test]
    fn sample_tests() {
        dotest(20, 27, true);
        dotest(12, 39, false);
        dotest(17, 34, false);
    }
}
