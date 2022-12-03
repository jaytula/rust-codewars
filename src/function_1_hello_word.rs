// https://www.codewars.com/kata/523b4ff7adca849afe000035/solutions/rust
fn greet() -> &'static str {
    "hello world!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greets_the_world() {
        assert_eq!(greet(), "hello world!", "should return the correct message");
    }
}