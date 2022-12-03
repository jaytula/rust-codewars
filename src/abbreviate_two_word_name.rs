// https://www.codewars.com/kata/57eadb7ecd143f4c9c0000a3/solutions/rust

fn abbrev_name(name: &str) -> String {
    let mut splitted = name.split_whitespace();
    let mut result = String::new();

    result.push(splitted.next().unwrap().chars().nth(0).unwrap());
    result.push('.');
    result.push(splitted.next().unwrap().chars().nth(0).unwrap());

    return result.to_uppercase();
}

// Rust test example:
#[test]
fn sample_tests() {
  assert_eq!(abbrev_name("Sam Harris"), "S.H");
  assert_eq!(abbrev_name("Patrick Feenan"), "P.F");
  assert_eq!(abbrev_name("Evan Cole"), "E.C");
  assert_eq!(abbrev_name("P Favuzzi"), "P.F");
  assert_eq!(abbrev_name("David Mendieta"), "D.M");
}