// https://www.codewars.com/kata/58841cb52a077503c4000015/solutions/rust

fn circle_of_numbers(n: u32, fst: u32) -> u32 {
    (fst + (n / 2 )) % n
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::circle_of_numbers;
        
    fn dotest(n: u32, fst: u32, expected: u32) {
        let actual = circle_of_numbers(n, fst);
        assert!(actual == expected, "With n = {n}, fst = {fst}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(10, 2, 7);
        dotest(10, 7, 2);
        dotest(4, 1, 3);
        dotest(6, 3, 0);
        dotest(20, 0, 10);
    }
}