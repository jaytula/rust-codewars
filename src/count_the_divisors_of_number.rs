// https://www.codewars.com/kata/542c0f198e077084c0000c2e/solutions/rust

fn divisors(n: u32) -> u32 {
    if n == 1 { return 1; }
    let mut total = 0u32;

    for i in 1..=((n as f32).powf(0.5) as usize) {
        if n % i as u32 != 0 { continue; }
        if (i * i) as u32 == n { total += 1; } else { total += 2; }
    }
    total
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::divisors;

    #[test]
    fn sample_tests() {
        assert_eq!(divisors(1), 1);
        assert_eq!(divisors(4), 3);
        assert_eq!(divisors(5), 2);
        assert_eq!(divisors(12), 6);
        assert_eq!(divisors(25), 3);
        assert_eq!(divisors(4096), 13);
    }
}
