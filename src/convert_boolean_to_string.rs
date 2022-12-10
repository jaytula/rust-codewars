// https://www.codewars.com/kata/551b4501ac0447318f0009cd/solutions/rust

fn boolean_to_string(b: bool) -> String {
    (if b { "true" } else { "false" }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            boolean_to_string(true),
            "true",
            "When we pass in true, we want the string \"true\" as output"
        );
        assert_eq!(
            boolean_to_string(false),
            "false",
            "When we pass in false, we want the string \"false\" as output"
        );
        assert_eq!(
            boolean_to_string(false),
            "false",
            "When we pass in false, we want the string \"false\" as output"
        );
    }
}
