// https://www.codewars.com/kata/5a4d303f880385399b000001/solutions/rust

fn factorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return 1u64;
    }
    return factorial(n - 1) * n;
}

fn strong(n: u64) -> String {
    let res = n.to_string().chars().fold(0u64, |acc, curr| {
        acc + factorial(curr.to_digit(10).unwrap() as u64)
    });
    (if res == n {
        "STRONG!!!!"
    } else {
        "Not Strong !!"
    })
    .to_string()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // Testing for 1
        assert_eq!(strong(1), "STRONG!!!!");

        // Testing for 2
        assert_eq!(strong(2), "STRONG!!!!");

        // Testing for 145
        assert_eq!(strong(145), "STRONG!!!!");

        // Testing for 7
        assert_eq!(strong(7), "Not Strong !!");

        // Testing for 93
        assert_eq!(strong(93), "Not Strong !!");

        // Testing for 185
        assert_eq!(strong(185), "Not Strong !!");
    }
}
