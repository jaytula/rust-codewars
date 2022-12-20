// https://www.codewars.com/kata/570523c146edc287a50014b1/solutions/rust

fn number_joy(n: u32) -> bool {
    let digit_sum = n
        .to_string()
        .chars()
        .map(|ch| ch.to_string().parse::<u32>().unwrap())
        .sum::<u32>();
    let digit_sum_reversed = digit_sum
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<u32>()
        .unwrap();
    digit_sum * digit_sum_reversed == n
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(n: u32, expected: bool, msg: &str) {
        assert_eq!(number_joy(n), expected, "{n}: {msg}");
    }

    #[test]
    fn test() {
        do_test(1997, false, "Not a Harshad number");
        do_test(
            1998,
            false,
            "Harshad but digit sum=27, and 27x72 does not equal 1998",
        );
        do_test(1729, true, "Harshad and digit sum=19, and 19x91 = 1729");
        do_test(
            18,
            false,
            "Harshad but digit sum=9, and 9x9 does not equal 18",
        );
        do_test(1, true, "Is a Harshad number");
        do_test(81, true, "Is a Harshad number");
        do_test(1458, true, "Is a Harshad number");
    }
}
