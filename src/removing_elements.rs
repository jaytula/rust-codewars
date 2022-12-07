// https://www.codewars.com/kata/5769b3802ae6f8e4890009d2/solutions/rust

fn remove_every_other(arr: &[u8]) -> Vec<u8> {
    let mut vec = Vec::new();

    for i in 0..arr.len() {
        if i % 2 == 1 { continue; }
        vec.push(arr[i])
    }

    vec
}

// Alternate strategies
// 1. iter, step_by, copied, collect
// 2. iter, step_by, cloned, collect
// 3. iter, enumerate, filter, map, collect
// 4. for loop with iter.enumerate and destructure tuple
// 5. declare vec using macro as vec![]
// 6. into_iter instead, appears to work the same

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::remove_every_other;

    #[test]
    fn sample_test() {
        assert_eq!(remove_every_other(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), &[1, 3, 5, 7, 9]);
    }
}
