// https://www.codewars.com/kata/577ff15ad648a14b780000e7/solutions/rust

fn greet(language: &str) -> &str {
    let langs = [("english", "Welcome"),
("czech", "Vitejte"),
("danish", "Velkomst"),
("dutch", "Welkom"),
("estonian", "Tere tulemast"),
("finnish", "Tervetuloa"),
("flemish", "Welgekomen"),
("french", "Bienvenue"),
("german", "Willkommen"),
("irish", "Failte"),
("italian", "Benvenuto"),
("latvian", "Gaidits"),
("lithuanian", "Laukiamas"),
("polish", "Witamy"),
("spanish", "Bienvenido"),
("swedish", "Valkommen"),
("welsh", "Croeso")];

  for &lang in langs.iter() {
    let (first, second) = lang;
    if language == first { return second; }
  }

  "Welcome"
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(greet("english"), "Welcome");
        assert_eq!(greet("dutch"), "Welkom");
        assert_eq!(greet("IP_ADDRESS_INVALID"), "Welcome");
        assert_eq!(greet(""), "Welcome");
        assert_eq!(greet("swelsh"), "Welcome");
    }
}