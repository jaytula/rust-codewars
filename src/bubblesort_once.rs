// https://www.codewars.com/kata/56b97b776ffcea598a0006f2/solutions/rust

fn bubblesort_once(lst: &[u32]) -> Vec<u32> {
    let mut vec = lst.to_vec();

    for i in 1..vec.len() {
        if vec[i - 1] < vec[i] {
            continue;
        }
        vec.swap(i - 1, i);
    }
    vec
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::bubblesort_once;

    fn dotest(a: &[u32], expected: &[u32]) {
        let actual = bubblesort_once(a);
        assert!(
            actual == expected,
            "With a = {a:?}\nExpected {expected:?} but got {actual:?}"
        )
    }

    #[test]
    fn example_test() {
        dotest(&[9, 7, 5, 3, 1, 2, 4, 6, 8], &[7, 5, 3, 1, 2, 4, 6, 8, 9]);
    }
}
