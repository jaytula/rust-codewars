// https://www.codewars.com/kata/58fa273ca6d84c158e000052/solutions/rust

fn digits(n: u64) -> usize {
    n.to_string().len()
}

#[test]
fn sample_test() {
    assert_eq!(digits(5), 1);
    assert_eq!(digits(12345), 5);
    assert_eq!(digits(9876543210), 10);
}
