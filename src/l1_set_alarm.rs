// https://www.codewars.com/kata/568dcc3c7f12767a62000038/solutions/rust

fn set_alarm(employed: bool, vacation: bool) -> bool {
    if employed && !vacation { return true; }
    false
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_alarm() {
        assert_eq!(set_alarm(true, true), false, "Fails when input is true, true");
        assert_eq!(set_alarm(false, true), false, "Fails when input is false, true");
        assert_eq!(set_alarm(false, false), false, "Fails when input is false, false");
        assert_eq!(set_alarm(true, false), true, "Fails when input is true, false");
    }
}
