// https://www.codewars.com/kata/56cafdabc8cfcc3ad4000a2b/solutions/rust

fn score(n: u32) -> u32 {
    let mut res: u32 = 0;
    for i in 0..32 {
        let bit = 1 << i;
        if bit <= n {
            res |= bit;
        }
    }
    res
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::score;

    #[test]
    fn fixed_tests() {
        assert_eq!(score(0), 0);
        assert_eq!(score(1), 1);
        assert_eq!(score(49), 63);
        assert_eq!(score(1000000), 1048575);
        assert_eq!(score(10000000), 16777215);
        assert_eq!(score(u32::MAX), u32::MAX);
    }
}
