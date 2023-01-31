// https://www.codewars.com/kata/56269eb78ad2e4ced1000013/solutions/rust

fn find_next_square(sq: u64) -> Option<u64> {
    let root = (sq as f64).sqrt();
    if root % 1.0 != 0.0 { return None; }
    Some(((root as u64) + 1).pow(2))
}

#[cfg(test)]
mod tests {
    use super::find_next_square;
    
    #[test]
    fn sample_tests() {
        assert_eq!(find_next_square(121), Some(144));
        assert_eq!(find_next_square(625), Some(676));
        assert_eq!(find_next_square(319_225), Some(320_356));
        assert_eq!(find_next_square(15_241_383_936), Some(15_241_630_849));
        assert_eq!(find_next_square(155), None);
        assert_eq!(find_next_square(342_786_627), None);
    }
}