// https://www.codewars.com/kata/57eba158e8ca2c8aba0002a0/solutions/rust

use itertools::Itertools;

fn sort_by_last_char(s: &str) -> Vec<String> {
    let mut s = s
        .split(" ")
        .map(|w| (w, w.chars().last().unwrap()))
        .collect_vec();
    s.sort_by(|a, b| a.1.cmp(&b.1));
    s.into_iter().map(|item| item.0.to_string()).collect_vec()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::sort_by_last_char;

    #[test]
    fn sample_tests() {
        assert_eq!(
            sort_by_last_char("man i need a taxi up to ubud"),
            vec!["a", "need", "ubud", "i", "taxi", "man", "to", "up"]
        );
        assert_eq!(
            sort_by_last_char("what time are we climbing up the volcano"),
            vec!["time", "are", "we", "the", "climbing", "volcano", "up", "what"]
        );
        assert_eq!(
            sort_by_last_char("take me to semynak"),
            vec!["take", "me", "semynak", "to"]
        );
        assert_eq!(
            sort_by_last_char("massage yes massage yes massage"),
            vec!["massage", "massage", "massage", "yes", "yes"]
        );
        assert_eq!(
            sort_by_last_char("take bintang and a dance please"),
            vec!["a", "and", "take", "dance", "please", "bintang"]
        );
    }
}
