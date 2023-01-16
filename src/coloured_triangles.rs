// https://www.codewars.com/kata/5a25ac6ac5e284cfbe000111/solutions/rust

fn triangle(row_str: &str) -> String {
    let mut s = row_str.to_string();

    while s.len() > 1 {
        s = s
            .chars()
            .zip(s.chars().skip(1))
            .map(|(a, b)| {
                if a == b {
                    a
                } else {
                    "RGB"
                        .replace(&a.to_string(), "")
                        .replace(&b.to_string(), "")
                        .chars()
                        .nth(0)
                        .unwrap()
                }
            })
            .collect();
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle() {
        assert_eq!(triangle("GB"), "R");
        assert_eq!(triangle("RRR"), "R");
        assert_eq!(triangle("RGBG"), "B");
        assert_eq!(triangle("RBRGBRB"), "G");
        assert_eq!(triangle("RBRGBRBGGRRRBGBBBGG"), "G");
        assert_eq!(triangle("GB"), "R");
        assert_eq!(triangle("B"), "B");
    }
}
