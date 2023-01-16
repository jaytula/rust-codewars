// https://www.codewars.com/kata/55fd2d567d94ac3bc9000064/solutions/rust

fn row_sum_odd_numbers(n: i64) -> i64 {
    n.pow(3)
}

#[test]
fn returns_expected() {
    assert_eq!(row_sum_odd_numbers(1), 1);
    assert_eq!(row_sum_odd_numbers(42), 74088);
}
