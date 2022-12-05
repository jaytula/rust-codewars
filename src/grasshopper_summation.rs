// https://www.codewars.com/kata/55d24f55d7dd296eb9000030/solutions/rust

fn summation(n: i32) -> i32 {
    (1..=n).sum()
}

#[cfg(test)]
mod tests {
    use super::summation;

    #[test]
    fn basic_tests() {
        assert_eq!(summation(1), 1);
        assert_eq!(summation(8), 36);
        assert_eq!(summation(22), 253);
        assert_eq!(summation(100), 5050);
        assert_eq!(summation(213), 22791);
    }
}