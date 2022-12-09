// https://www.codewars.com/kata/57f6ad55cca6e045d2000627/solutions/rust

fn square_or_square_root(arr: &[u32]) -> Vec<u32> {
    arr.iter().map(|&num| {
        let root = (num as f32).sqrt() as u32;
        if root.pow(2) == num { root } else { num.pow(2) }

    }).collect()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::square_or_square_root;

    fn dotest(arr: &[u32], expected: &[u32]) {
        let actual = square_or_square_root(arr);
        assert!(actual == expected, "Test failed with  arr = {arr:?}\nExpected {expected:?} but got {actual:?}");
    }

    #[test]
    fn sample_tests() {
        dotest(&[4, 3, 9, 7, 2, 1], &[2, 9, 3, 49, 4, 1]);
        dotest(&[100, 101, 5, 5, 1, 1], &[10, 10201, 25, 25, 1, 1]);
        dotest(&[1, 2, 3, 4, 5, 6], &[1, 4, 9, 2, 25, 36]);
    }
}
