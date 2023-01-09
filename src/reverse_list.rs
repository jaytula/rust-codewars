// https://www.codewars.com/kata/57a04da9e298a7ee43000111/solutions/rust

fn reverse_list(lst: &[i32]) -> Vec<i32> {
    lst.iter().rev().map(|el| *el).collect()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::reverse_list;
        
    fn dotest(a: &[i32], expected: &[i32]) {
        let actual = reverse_list(a);
        assert!(actual == expected, "With lst = {a:?}\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn sample_tests() {
        dotest(&[], &[]);
        dotest(&[1,2,3,4], &[4,3,2,1]);
        dotest(&[2,3,4,5,6], &[6,5,4,3,2]);
    }
}
