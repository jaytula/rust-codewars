// https://www.codewars.com/kata/53ee5429ba190077850011d4/solutions/rust

fn double_integer(n: i32) -> i32 {
    n * 2
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::double_integer;

    #[test]
    fn sample_tests() {
        assert_eq!(double_integer(1), 2);
        assert_eq!(double_integer(5), 10);
        assert_eq!(double_integer(10), 20);
        assert_eq!(double_integer(20), 40);
        assert_eq!(double_integer(40), 80);
    }
}