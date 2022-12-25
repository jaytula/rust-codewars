// https://www.codewars.com/kata/5a03b3f6a1c9040084001765/train/rust

fn angle(n: u32) -> u32 {
    (n - 2) * 180
}

#[test]
fn normal_tests() {
    assert_eq!(angle(3), 180);
    assert_eq!(angle(4), 360);
    assert_eq!(angle(5), 540);
}
