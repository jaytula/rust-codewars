// https://www.codewars.com/kata/5813d19765d81c592200001a/solutions/rust

fn dont_give_me_five(start: isize, end: isize) -> isize {
    (start..=end)
        .filter(|num| !num.to_string().contains("5"))
        .count() as isize
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[test]
fn returns_expected() {
    assert_eq!(dont_give_me_five(1, 9), 8);
    assert_eq!(dont_give_me_five(4, 17), 12);
}
