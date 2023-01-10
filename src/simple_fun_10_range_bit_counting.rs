fn range_bit_count(a: u32, b: u32) -> u32 {
    (a..=b)
        .map(|num| format!("{:b}", num).chars().filter(|ch| *ch == '1').count() as u32)
        .sum()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::range_bit_count;

    fn dotest(a: u32, b: u32, expected: u32) {
        let actual = range_bit_count(a, b);
        assert!(
            actual == expected,
            "With a = {a}, b = {b}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(2, 7, 11);
        dotest(0, 1, 1);
        dotest(4, 4, 1);
    }
}
