fn get_sum(a: i64, b: i64) -> i64 {
    let mut v = [a, b].to_vec();
    v.sort();
    (v[0]..=v[1]).sum()
}

// See: https://doc.rust-lang.org/book/testing.html

#[cfg(test)]
mod tests {
    use super::get_sum;

    #[test]
    fn sample_tests() {
        assert_eq!(get_sum(0, 1), 1);
        assert_eq!(get_sum(1, 2), 3);
        assert_eq!(get_sum(5, -1), 14);
        assert_eq!(get_sum(505, 4), 127759);
    }
}
