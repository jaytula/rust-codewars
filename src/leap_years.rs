// https://www.codewars.com/kata/526c7363236867513f0005ca/solutions/rust

fn is_leap_year(year: i32) -> bool {
    if year % 400 == 0 { return true; }
    if year % 4 != 0 { return false; }
    if year % 100 == 0 { return false; }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn sample_tests() {
        assert_eq!(is_leap_year(1234), false);
        assert_eq!(is_leap_year(1984), true);
        assert_eq!(is_leap_year(2000), true);
        assert_eq!(is_leap_year(2010), false);
        assert_eq!(is_leap_year(2013), false);
    }    
}