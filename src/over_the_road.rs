// https://www.codewars.com/kata/5f0ed36164f2bc00283aed07/solutions/rust

fn over_the_road(address: u64, n: u64) -> u64 {
    n * 2 + 1 - address
}

#[cfg(test)]
mod tests {
    use super::over_the_road;

    #[test]
    fn basic() {
        assert_eq!(over_the_road(1, 3), 6);
        assert_eq!(over_the_road(3, 3), 4);
        assert_eq!(over_the_road(2, 3), 5);
        assert_eq!(over_the_road(3, 5), 8);
        assert_eq!(over_the_road(7, 11), 16);
        assert_eq!(over_the_road(20, 1_000_000), 1_999_981);
        assert_eq!(
            over_the_road(23_633_656_673, 310_027_696_726),
            596_421_736_780
        );
    }
}
