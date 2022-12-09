// https://www.codewars.com/kata/55cbd4ba903825f7970000f5/solutions/rust

fn get_grade(s1: u16, s2: u16, s3: u16) -> char {
    let total = (s1 + s2 + s3) as f32 / 3.0;

    if total >= 90.0 { return 'A'; }
    if total >= 80.0 { return 'B'; }
    if total >= 70.0 { return 'C'; }
    if total >= 60.0 { return 'D'; }
    'F'
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::get_grade;

    fn dotest(s1: u16, s2: u16, s3: u16, expected: char) {
        let actual = get_grade(s1, s2, s3);
        assert!(actual == expected, 
            "With s1 = {s1}, s2 = {s2}, s = {s3}\nExpected '{expected}' but got '{actual}'")
    }
    
    #[test]
    fn test_get_grade() {
        dotest(100,100,100, 'A');
        dotest(95,90,93, 'A');
        dotest(82,85,87, 'B');
        dotest(60,82,76, 'C');
        dotest(58,62,70, 'D');
        dotest(58,59,60, 'F');
        dotest(0,0,0, 'F');
    }
}
