// https://www.codewars.com/kata/515e271a311df0350d00000f/solutions/rust

fn square_sum(vec: Vec<i32>) -> i32 {
    vec.iter().fold(0, |acc, num| { acc + num*num })
}

#[test]
fn returns_expected() {
    assert_eq!(square_sum(vec![1, 2]), 5);
    assert_eq!(square_sum(vec![-1, -2]), 5);
    assert_eq!(square_sum(vec![5, 3, 4]), 50);
    assert_eq!(square_sum(vec![]), 0);
}