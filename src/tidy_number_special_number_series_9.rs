// https://www.codewars.com/kata/5a87449ab1710171300000fd/solutions/rust

fn tidy_number(n: u64) -> bool {
    let mut min = 0;
    for ch in n.to_string().chars() {
        if ch.to_digit(10).unwrap() < min { return false; }
        min = ch.to_digit(10).unwrap();
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(tidy_number(12), true);
        assert_eq!(tidy_number(102), false);
        assert_eq!(tidy_number(9672), false);
        assert_eq!(tidy_number(2789), true);
        assert_eq!(tidy_number(2335), true);
    }
}
