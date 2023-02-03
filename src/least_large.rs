// https://www.codewars.com/kata/5f8341f6d030dc002a69d7e4/solutions/rust

fn least_larger(a: &[i32], i: usize) -> Option<usize> {
    let mut res = None;

    for (idx, num) in a.iter().enumerate() {
        if num > &a[i] && (res == None || num < &a[res.unwrap()]) {
            res = Some(idx);
        }
    }
    res
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::least_larger;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(a: &[i32], i: usize, expected: Option<usize>) {
        assert_eq!(least_larger(a, i), expected, "{ERR_MSG} with a = {a:?}, i = {i}")   
    }

    #[test]
    fn sample_tests() {
        dotest(&[4, 1, 3, 5, 6], 0, Some(3));
        dotest(&[4, 1, 3, 5, 6], 4, None);
        dotest(&[1, 3, 5, 2, 4], 0, Some(3));
    }
}
