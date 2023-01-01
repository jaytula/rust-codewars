// https://www.codewars.com/kata/59706036f6e5d1e22d000016/solutions/rust

fn words_to_marks(s: &str) -> u32 {
    s.chars()
        .fold(0u32, |acc, curr| acc + 1 + (curr as u32 - 'a' as u32))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(words_to_marks("attitude"), 100);
        assert_eq!(words_to_marks("friends"), 75);
        assert_eq!(words_to_marks("family"), 66);
        assert_eq!(words_to_marks("selfness"), 99);
        assert_eq!(words_to_marks("knowledge"), 96);
    }
}
