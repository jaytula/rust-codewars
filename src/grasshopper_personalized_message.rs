// https://www.codewars.com/kata/5772da22b89313a4d50012f7/solutions/rust

fn greet(name: &str, owner: &str) -> String {
    format!("Hello {}", if name == owner {"boss"} else {"guest"})
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("Daniel", "Daniel"), "Hello boss");
        assert_eq!(greet("Greg", "Daniel"), "Hello guest");
    }
}
