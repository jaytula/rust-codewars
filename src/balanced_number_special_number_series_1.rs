// https://www.codewars.com/kata/5a4e3782880385ba68000018/solutions/rust

fn balanced_num(n: u64) -> String {
    let n = n.to_string();

    let is_odd = n.len() % 2 == 1;

    let mid1 = if is_odd {n.len() / 2} else {(n.len() / 2) - 1};
    let mid2 = if is_odd {1 + n.len() / 2} else {(n.len() / 2) + 1};

    let sum1 = n[0..mid1]
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .sum::<u32>();
    let sum2 = n[(mid2)..(n.len())]
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .sum::<u32>();

    let res = sum1 == sum2;
    (if res { "Balanced" } else { "Not Balanced" }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balanced_number() {
        assert_eq!(balanced_num(7), "Balanced");
        assert_eq!(balanced_num(959), "Balanced");
        assert_eq!(balanced_num(13), "Balanced");
        assert_eq!(balanced_num(432), "Not Balanced");
        assert_eq!(balanced_num(424), "Balanced");
    }

    #[test]
    fn larger_number() {
        assert_eq!(balanced_num(1024), "Not Balanced");
        assert_eq!(balanced_num(66545), "Not Balanced");
        assert_eq!(balanced_num(295591), "Not Balanced");
        assert_eq!(balanced_num(1230987), "Not Balanced");
        assert_eq!(balanced_num(56239814), "Balanced");
    }
}
