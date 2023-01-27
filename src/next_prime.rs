// https://www.codewars.com/kata/58e230e5e24dde0996000070/solutions/rust

fn next_prime(n: u64) -> u64 {
    let is_prime = |num: u64| {
        for f in 2u64..=(num as f64).sqrt().ceil() as u64 {
            if num % f == 0 {
                return false;
            }
        }
        true
    };
    if n < 2 {
        return 2;
    }
    let mut candidate = n + 1;

    loop {
        if is_prime(candidate) {
            return candidate;
        }
        candidate += 1;
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::next_prime;

    fn dotest(n: u64, expected: u64) {
        let actual = next_prime(n);
        assert!(
            actual == expected,
            "With n = {n}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(0, 2);
        dotest(2, 3);
        dotest(3, 5);
        dotest(13, 17);
        dotest(181, 191);
        dotest(911, 919);
    }
}
