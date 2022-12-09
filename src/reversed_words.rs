// https://www.codewars.com/kata/51c8991dee245d7ddf00000e/solutions/rust

fn reverse_words(words: &str) -> String {
    let split_revved = words.split_whitespace().rev();
    let mut res = String::from("");
    for (i, word) in split_revved.enumerate() {
        if i != 0 { res.push(' '); };
        res.push_str(word);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::reverse_words;
    #[test]
    fn returns_expected() {
        assert_eq!(reverse_words("hello world!"), "world! hello");
    }
}