// https://www.codewars.com/kata/5d50e3914861a500121e1958/solutions/rust

fn add_letters(letters: Vec<char>) -> char {
    let base = 'a' as u32 - 1;

    let sum: u32 = letters.iter().map(|&ch| (ch as u32 - base) % 26).sum();
    let sum = sum % 26;

    if sum == 0 {
        'z'
    } else {
        std::char::from_u32(base + sum).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(add_letters(vec!['a', 'b', 'c']), 'f');
        assert_eq!(add_letters(vec!['z']), 'z');
        assert_eq!(add_letters(vec!['a', 'b']), 'c');
        assert_eq!(add_letters(vec!['c']), 'c');
        assert_eq!(add_letters(vec!['z', 'a']), 'a');
        assert_eq!(add_letters(vec!['y', 'c', 'b']), 'd');
        assert_eq!(add_letters(vec![]), 'z');
    }
}
