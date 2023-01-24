// https://www.codewars.com/kata/59a8570b570190d313000037/solutions/rust

fn sum_cubes(n: u32) -> u32 {
    (1..=n).fold(0, |acc, curr| acc + curr.pow(3))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(sum_cubes(1), 1);
        assert_eq!(sum_cubes(2), 9);
        assert_eq!(sum_cubes(3), 36);
        assert_eq!(sum_cubes(4), 100);
        assert_eq!(sum_cubes(10), 3_025);
        assert_eq!(sum_cubes(123), 58_155_876);
    }
}
