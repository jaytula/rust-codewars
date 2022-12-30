// https://www.codewars.com/kata/55908aad6620c066bc00002a/solutions/rust

fn xo(string: &'static str) -> bool {
    let o_count = string.to_lowercase().chars().filter(|ch| *ch == 'o').count();
    let x_count = string.to_lowercase().chars().filter(|ch| *ch == 'x').count();

    o_count == x_count
}

#[test]
fn returns_expected() {
    assert_eq!(xo("xo"), true);
    assert_eq!(xo("Xo"), true);
    assert_eq!(xo("xxOo"), true);
    assert_eq!(xo("xxxm"), false);
    assert_eq!(xo("Oo"), false);
    assert_eq!(xo("ooom"), false);
}
