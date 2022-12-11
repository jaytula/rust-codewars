// https://www.codewars.com/kata/57eae65a4321032ce000002d/solutions/rust

fn fake_bin(s: &str) -> String {
    s.chars().map(|ch| {
        match ch {
            '0'..='4' => '0',
            '5'..='9' => '1',
            _ => panic!("Invalid character!"),
        }
    }).collect()
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::fake_bin;

    #[test]
    fn basic_tests() {
        assert_eq!(fake_bin("45385593107843568"), "01011110001100111");
        assert_eq!(fake_bin("509321967506747"), "101000111101101");
        assert_eq!(fake_bin("366058562030849490134388085"), "011011110000101010000011011");
        assert_eq!(fake_bin("15889923"), "01111100");
        assert_eq!(fake_bin("800857237867"), "100111001111");
    }
}