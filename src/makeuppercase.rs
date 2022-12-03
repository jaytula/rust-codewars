// https://www.codewars.com/kata/57a0556c7cb1f31ab3000ad7/solutions/rust

fn make_upper_case(s: &str) -> String {
    s.to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_upper_case() {
        assert_eq!(make_upper_case("hello"), "HELLO");
    }
}