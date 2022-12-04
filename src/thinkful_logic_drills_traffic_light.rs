// https://www.codewars.com/kata/58649884a1659ed6cb000072/solutions/rust

fn update_light(current: &str) -> String {
    match current {
        "green" => String::from("yellow"),
        "yellow" => "red".to_string(),
        "red" => "green".to_string(),
        _ => String::from(""),
    }
}

#[test]
fn basic_test() {
  assert_eq!(update_light("green"), "yellow");
  assert_eq!(update_light("yellow"), "red");
  assert_eq!(update_light("red"), "green");
}