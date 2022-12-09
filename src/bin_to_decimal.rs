// https://www.codewars.com/kata/57a5c31ce298a7e6b7000334/solutions/rust

fn bin_to_decimal(inp: &str) -> i32 {
    let mut res = 0i32;
    for (i, ch) in inp.chars().rev().enumerate() {
        if ch == '1' { res += 2i32.pow(i as u32) }
    }
    res
}

#[test]
fn test_bin_to_decimal() {
    assert_eq!(bin_to_decimal("0"), 0);
    assert_eq!(bin_to_decimal("1"), 1);
    assert_eq!(bin_to_decimal("10"), 2);
    assert_eq!(bin_to_decimal("1001001"), 73);
    assert_eq!(bin_to_decimal("1101000000111001101001"), 3411561)
}