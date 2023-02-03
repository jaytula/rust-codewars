// https://www.codewars.com/kata/589478160c0f8a40870000bc/solutions/rust

fn arrow_area(a: u32, b: u32) -> f64 {
    0.25 * b as f64 * a as f64
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::arrow_area;
        
    fn dotest(a: u32, b: u32, expected: f64) {
        let actual = arrow_area(a, b);
        assert!(actual == expected, "With a = {a}, b = {b}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(4, 2, 2.0);
        dotest(7, 6, 10.5);
        dotest(25, 25, 156.25);
    }
}
