// https://www.codewars.com/kata/57e3f79c9cb119374600046b/solutions/rust

fn hello(name: &str) -> String {
    if name == "" { return "Hello, World!".to_string(); };

    let name = String::from(name);

    format!("Hello, {}{}!", &name[..1].to_uppercase(), &name[1..].to_lowercase())
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(hello("johN"), "Hello, John!");
        assert_eq!(hello("alice"), "Hello, Alice!");
        assert_eq!(hello(""), "Hello, World!");
    }
}
