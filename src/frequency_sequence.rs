// https://www.codewars.com/kata/585a033e3a36cdc50a00011c/solutions/rust

use std::collections::HashMap;
use itertools::Itertools;

fn freq_seq(s: &str, sep: &str) -> String {
    let mut hs = HashMap::new();

    for i in s.chars() {
        if !hs.contains_key(&i) {
            hs.insert(i, 0);
        }
        *hs.get_mut(&i).unwrap() += 1;
    }

    s.chars().map(|ch| hs[&ch].to_string()).join(sep)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(freq_seq("hello world", "-"), "1-1-3-3-2-1-1-2-1-3-1");
        assert_eq!(freq_seq("19999999", ":"), "1:7:7:7:7:7:7:7");
        assert_eq!(freq_seq("^^^**$", "x"), "3x3x3x2x2x1");
    }
}
