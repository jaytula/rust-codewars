// https://www.codewars.com/kata/56747fd5cb988479af000028/solutions/rust

fn get_middle(s:&str) -> &str {
    let length = s.len();

    if length % 2 == 0 {
        &s[(length / 2 - 1)..(length / 2 + 1)]
    } else {
        &s[(length / 2)..(length / 2 + 1)]
    }
}

#[test]
fn example_tests() {
  assert_eq!(get_middle("test"),"es");
  assert_eq!(get_middle("testing"),"t");
  assert_eq!(get_middle("middle"),"dd");
  assert_eq!(get_middle("A"),"A");
  assert_eq!(get_middle("of"),"of");
}