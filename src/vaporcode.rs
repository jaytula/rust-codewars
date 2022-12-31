// https://www.codewars.com/kata/5966eeb31b229e44eb00007a/solutions/rust

use itertools::Itertools;

fn vaporcode(s: &str) -> String {
    s.to_uppercase().replace(" ", "").chars().join("  ")
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(vaporcode("Lets go to the movies"), "L  E  T  S  G  O  T  O  T  H  E  M  O  V  I  E  S".to_string());
        assert_eq!(vaporcode("Why isn't my code working?"), "W  H  Y  I  S  N  '  T  M  Y  C  O  D  E  W  O  R  K  I  N  G  ?".to_string());
    }
}
