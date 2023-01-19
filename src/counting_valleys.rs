// https://www.codewars.com/kata/5da9973d06119a000e604cb6/solutions/rust

fn counting_valleys(s: &str) -> u32 {
    let mut count: u32 = 0;
    let mut level: i32 = 0;
    let mut in_valley: bool = false;

    for ch in s.chars() {
        if ch == 'F' { continue; }
        if in_valley && level == -1 && ch == 'U' {
            in_valley = false;
            level = 0;
            count += 1;
            continue;
        }
        if !in_valley && level == 0 && ch == 'D' {
            in_valley = true;
            level = -1;
            continue;
        }
        level += if ch == 'U' { 1 } else { -1 };
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_tests() {
        assert_eq!(counting_valleys("UFFDDFDUDFUFU"), 1);
        assert_eq!(counting_valleys("UFFDDFDUDFUFUUFFDDUFFDDUFFDDUDUDUDUDUDUUUUUUUUU"), 3);
        assert_eq!(counting_valleys("UFFDDFDUDFUFUUFFDDFDUDFUFUUFFDDFDUDFUFUUFFDDFDUDFUFUUFFDDFDUDFUFUUFFDDFDUDFUFU"), 6);
        assert_eq!(counting_valleys("UFFDDFDUDFUFUUFFDDFDUDFUFU"), 2);
        assert_eq!(counting_valleys("UFFDDFDUDFUFUUFFDDFDUDFUFUUFFDDFDUDFUFUUFFDDFDUDFUFU"), 4);
        assert_eq!(counting_valleys("DFFFU"), 1);
        assert_eq!(counting_valleys("UFFFD"), 0);
        assert_eq!(counting_valleys("DFFFD"), 0);
        assert_eq!(counting_valleys("UFFFU"), 0);
    }
}
