// https://www.codewars.com/kata/5583090cbe83f4fd8c000051/solutions/rust

fn digitize(n: u64) -> Vec<u8> {
    format!("{}", n)
        .chars()
        .rev()
        .map(|ch| ch.to_digit(10).unwrap_or(0) as u8)
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(digitize(348597), vec![7, 9, 5, 8, 4, 3]);
        assert_eq!(digitize(35231), vec![1, 3, 2, 5, 3]);
        assert_eq!(digitize(0), vec![0]);
    }
}
