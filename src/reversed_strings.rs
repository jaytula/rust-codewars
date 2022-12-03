// https://www.codewars.com/kata/5168bb5dfe9a00b126000018/solutions/rust
fn solution(phrase: &str) -> String {
    let mut s = String::new();

    for ch in phrase.chars().rev() {
        s.push(ch);
    }

    return s;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_test() {
        assert_eq!(solution("world"), "dlrow");
    }
}
