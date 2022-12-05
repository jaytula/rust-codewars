// https://www.codewars.com/kata/571d42206414b103dc0006a1/solutions/rust

fn arr(n: usize) -> Vec<u32> {
    (0..n).map(|i| {i as u32}).collect()
}

#[cfg(test)]
mod tests {
    use super::arr;

    #[test]
    fn example_tests() {
        assert_eq!(arr(0), vec![]);
        assert_eq!(arr(4), vec![0,1,2,3]);
    }
}