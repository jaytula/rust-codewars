// https://www.codewars.com/kata/546e2562b03326a88e000020/solutions/rust

fn square_digits(num: u64) -> u64 {
    let num = num.to_string();
    let num: Vec<String> = num
        .chars()
        .map(|ch| ch.to_string().parse::<u64>().unwrap().pow(2).to_string())
        .collect();
    num.join("").parse::<u64>().unwrap()
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::square_digits;

    #[test]
    fn test_square_digits() {
        assert_eq!(square_digits(9119), 811181, "\nFailed with num 9119");
    }
}
