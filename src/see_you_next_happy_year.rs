// https://www.codewars.com/kata/5ae7e3f068e6445bc8000046/solutions/rust

fn next_happy_year(year: u16) -> u16 {
    let mut year = year + 1;
    let is_happy = |num: u16| {
        let mut v: Vec<char> = num.to_string().chars().collect();
        v.sort();
        v.dedup();
        v.len() == 4
    };
    loop {
        if is_happy(year) { return year; }
        year += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(next_happy_year(1001), 1023);
        assert_eq!(next_happy_year(1123), 1203);
        assert_eq!(next_happy_year(2001), 2013);
        assert_eq!(next_happy_year(2334), 2340);
        assert_eq!(next_happy_year(3331), 3401);
        assert_eq!(next_happy_year(1987), 2013);
        assert_eq!(next_happy_year(5555), 5601);
        assert_eq!(next_happy_year(7712), 7801);
        assert_eq!(next_happy_year(8088), 8091);
        assert_eq!(next_happy_year(8999), 9012);
    }
}
