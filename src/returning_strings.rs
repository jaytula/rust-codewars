// https://www.codewars.com/kata/55a70521798b14d4750000a4/solutions/rust

fn greet(name: &str) -> String {
    format!("Hello, {} how are you doing today?", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(greet("Ryan"), "Hello, Ryan how are you doing today?");
        assert_eq!(
            greet("Shingles"),
            "Hello, Shingles how are you doing today?"
        );
    }
}
