// https://www.codewars.com/kata/56ba65c6a15703ac7e002075/solutions/rust

fn find_next_power(val: u64, pow_: u32) -> u64 {
    for i in 1..u64::MAX {
        let res = i.pow(pow_);
        if res > val { return res; }
    }
    u64::MAX
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::find_next_power;

    fn dotest(n: u64, p: u32, expected: u64) {
        let actual = find_next_power(n, p);
        assert!(
            actual == expected,
            "With val = {n}, pow_ = {p}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(12385, 3, 13824);
        dotest(1245678, 5, 1419857);
        dotest(1245678, 6, 1771561);
        dotest(1000000, 6, 1771561);
    }
}
