// https://www.codewars.com/kata/57f24e6a18e9fad8eb000296/solutions/rust

fn how_much_i_love_you(nb_petals: u16) -> &'static str {
    let phrases = ["I love you", "a little", "a lot", "passionately", "madly", "not at all"];
    phrases[(nb_petals-1) as usize % phrases.len()]
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::how_much_i_love_you;

    #[test]
    fn fixed_tests() {
        assert_eq!(how_much_i_love_you(7), "I love you");
        assert_eq!(how_much_i_love_you(3), "a lot");
        assert_eq!(how_much_i_love_you(6), "not at all");
    }
}
