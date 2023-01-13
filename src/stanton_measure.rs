// https://www.codewars.com/kata/59a1cdde9f922b83ee00003b/solutions/rust

fn stanton_measure(arr: &[u32]) -> u32 {
    let ones_count = arr.iter().filter(|&el| *el == 1).count() as u32;
    arr.iter().filter(|&el| *el == ones_count).count() as u32
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::stanton_measure;
    
    fn dotest(arr: &[u32], expected: u32) {
        let actual = stanton_measure(arr);
        assert!(actual == expected, "With arr = {arr:?}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[1, 4, 3, 2, 1, 2, 3, 2], 3);
    }
}
