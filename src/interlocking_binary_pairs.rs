// https://www.codewars.com/kata/628e3ee2e1daf90030239e8a/solutions/rust

fn interlockable(a: u64, b: u64) -> bool {
    let res = a.checked_add(b);
    match res {
        None => false,
        _ => a+b == a|b,
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::interlockable;
        
    fn dotest(a: u64, b: u64, expected: bool) {
        let actual = interlockable(a, b);
        assert!(actual == expected, "With\na = {a}\nb = {b}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn sample_tests() {
        for (a, b, expected) in [(9, 4, true),
                                 (3, 6, false),
                                 (2, 5, true),
                                 (7, 1, false),
                                 (0, 8, true)]
        {
            dotest(a, b, expected);
        }
    }
}
