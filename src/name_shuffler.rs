// https://www.codewars.com/kata/559ac78160f0be07c200005a/solutions/rust

fn name_shuffler(s: &str) -> String {
    let mut reversed = s.split_whitespace().rev();
    let mut s = String::from("");
    s.push_str(reversed.next().unwrap());
    s.push(' ');
    s.push_str(reversed.next().unwrap());
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(name_shuffler("john McClane"), "McClane john");
        assert_eq!(name_shuffler("Mary jeggins"), "jeggins Mary");
        assert_eq!(name_shuffler("tom jerry"), "jerry tom");
    }
}