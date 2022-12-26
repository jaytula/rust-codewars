// https://www.codewars.com/kata/56b8903933dbe5831e000c76/solutions/rust

fn spoonerize(words: &str) -> String {
    let (word1, word2) = words.split_once(" ").unwrap();

    let word1_initial = &word1[0..1];
    let word2_initial = &word2[0..1];

    let mut word1 = word1.to_string();
    let mut word2 = word2.to_string();

    word1.replace_range(0..1, word2_initial);
    word2.replace_range(0..1, word1_initial);

    format!("{} {}", word1, word2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(spoonerize("nit picking"), "pit nicking");
        assert_eq!(spoonerize("wedding bells"), "bedding wells");
        assert_eq!(spoonerize("jelly beans"), "belly jeans");
    }
}
