// https://www.codewars.com/kata/5a8059b1fd577709860000f6/solutions/rust

fn alphabetic(s: &str) -> bool {
    let mut letters: Vec<u32> = s.chars().map(|ch| ch as u32).collect();
    letters.sort();
    letters
        .iter()
        .map(|&ch| std::char::from_u32(ch).unwrap())
        .collect::<String>()
        == s.to_string()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::alphabetic;

    fn dotest(s: &str, expected: bool) {
        let actual = alphabetic(s);
        assert!(
            actual == expected,
            "With s = \"{s}\"\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest("asd", false);
        dotest("codewars", false);
        dotest("door", true);
        dotest("cell", true);
        dotest("z", true);
        dotest("", true);
    }
}
