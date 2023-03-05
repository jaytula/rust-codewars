// https://www.codewars.com/kata/5500d54c2ebe0a8e8a0003fd/solutions/rust

fn gcd(a: u32, b: u32) -> u32 {
    let min = a.min(b);
    let mut vec: Vec<u32> = Vec::new();
    let upper_limit = (min as f32).sqrt().ceil() as u32;
    let mut greatest: u32 = 1;

    for i in 2..=upper_limit {
        if a % i == 0 && b % i == 0 {
            vec.push(i)
        }
    }

    for i in vec.iter() {
        while (a % (greatest * i) == 0) && (b % (greatest * i) == 0) {
            greatest *= i;
        }
    }

    greatest
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(gcd(30, 12), 6);
        assert_eq!(gcd(8, 9), 1);
        assert_eq!(gcd(1, 1), 1);
    }
}
