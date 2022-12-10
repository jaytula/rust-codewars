// https://www.codewars.com/kata/5933a1f8552bc2750a0000ed/solutions/rust

fn nth_even(n: u32) -> u32 {
    let mut res = 0;

    for _ in 0..(n-1) {
        res += 2;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn example_tests() {
        assert_eq!(nth_even(1), 0);
        assert_eq!(nth_even(2), 2);
        assert_eq!(nth_even(3), 4);
        assert_eq!(nth_even(100), 198);
        assert_eq!(nth_even(1298734), 2597466);
    }
}