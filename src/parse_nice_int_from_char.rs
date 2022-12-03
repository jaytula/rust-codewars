// https://www.codewars.com/kata/557cd6882bfa3c8a9f0000c1/solutions/rust

fn get_age(age: &str) -> u32 {
    let words: Vec<&str> = age.split(" ").collect();
    words[0].parse().unwrap()
}

#[test]
fn basic_tests() {
    assert_eq!(get_age("2 years old"), 2);
    assert_eq!(get_age("4 years old"), 4);
    assert_eq!(get_age("5 years old"), 5);
    assert_eq!(get_age("7 years old"), 7);
}
