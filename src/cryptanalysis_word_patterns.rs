// https://www.codewars.com/kata/5f3142b3a28d9b002ef58f5e/solutions/rust

use std::collections::HashMap;
use itertools::Itertools;

fn word_pattern(word: &str) -> String {
    let word = word.to_ascii_lowercase();
    let mut m: HashMap<char, i32> = HashMap::new();
    let mut idx = 0;

    for ch in word.chars() {
        if m.contains_key(&ch) {
            continue;
        }
        m.insert(ch.clone(), idx);
        idx += 1;
    }

    let res = word
        .chars()
        .map(|ch| format!("{}", m.get(&ch).unwrap()))
        .join(".");
    res
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::word_pattern;

    fn dotest(s: &str, expected: &str) {
        let actual = word_pattern(s);
        assert!(
            actual == expected,
            "With word = \"{s}\"\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest("hello", "0.1.2.2.3");
        dotest("heLlo", "0.1.2.2.3");
        dotest("helLo", "0.1.2.2.3");
        dotest(
            "Hippopotomonstrosesquippedaliophobia",
            "0.1.2.2.3.2.3.4.3.5.3.6.7.4.8.3.7.9.7.10.11.1.2.2.9.12.13.14.1.3.2.0.3.15.1.13",
        );
    }
}
