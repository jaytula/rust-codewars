// https://www.codewars.com/kata/5b180e9fedaa564a7000009a/solutions/rust

fn solve(s: &str) -> String {
    let lower_count = s
        .chars()
        .filter(|ch| !ch.is_uppercase())
        .collect::<String>()
        .len();
    if lower_count >= s.len() / 2 {
        s.to_lowercase()
    } else {
        s.to_uppercase()
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solve("code"), "code");
        assert_eq!(solve("CODe"), "CODE");
        assert_eq!(solve("COde"), "code");
        assert_eq!(solve("Code"), "code");
    }
}
