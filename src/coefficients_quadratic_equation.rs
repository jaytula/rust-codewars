// https://www.codewars.com/kata/5d59576768ba810001f1f8d6/solutions/rust
pub fn quadratic(x1: i32, x2: i32) -> (i32, i32, i32) {
    (1, -(x1+x2), x1*x2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(quadratic(0, 1), (1, -1, 0));
        assert_eq!(quadratic(1, 1), (1, -2, 1));
        assert_eq!(quadratic(-4, -9), (1, 13, 36));
        assert_eq!(quadratic(-5, -4), (1, 9, 20));
        assert_eq!(quadratic(4, -9), (1, 5, -36));
        assert_eq!(quadratic(5, -4), (1, -1, -20));
    }
}
