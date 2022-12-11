// https://www.codewars.com/kata/55a2d7ebe362935a210000b2/solutions/rust

fn find_smallest_int(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}

// Rust test example:
#[test]
fn sample_tests() {
    assert_eq!(find_smallest_int(&[34, 15, 88, 2]), 2);
    assert_eq!(find_smallest_int(&[34, -345, -1, 100]), -345);
}