// https://www.codewars.com/kata/634d0723075de3f97a9eb604/solutions/rust

pub fn encode(s: String) -> String {
    let mut res = String::new();

    for i in 0..(s.len()/2) {
        res.push(s.chars().nth(i).unwrap());
        res.push(s.chars().nth(s.len() - i - 1).unwrap());
    }
    if s.len() % 2 == 1 {
        res.push(s.chars().nth(s.len() / 2).unwrap());
    }
    res
}

pub fn decode(s: String) -> String {
    let mut res = String::new();

    for (i, ch) in s.chars().enumerate() {
        if i % 2 == 1 { continue; }
        res.push(ch);
    }

    let offset = if s.len() % 2 == 0 { 0 } else { 1 };
    for (i, ch) in s.chars().rev().enumerate() {
        if (i % 2) +offset == 1 { continue; }
        res.push(ch);
    }
    res
}

// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_basic_tests() {
        assert_eq!("csordaew".to_string(), encode("codewars".to_string()));
        assert_eq!("wehti".to_string(), encode("white".to_string()));
        assert_eq!("Atsrse".to_string(), encode("Assert".to_string()));
        assert_eq!("H!edlllroo w".to_string(), encode("Hello world!".to_string()));
        assert_eq!("Y.oaut ahka vsei hcth oesteanl stnoa rt".to_string(), encode("You have chosen to translate this kata.".to_string()));
    }
    
    #[test]
    fn decode_basic_tests() {
        assert_eq!("codewars".to_string(), decode("csordaew".to_string()));
        assert_eq!("white".to_string(), decode("wehti".to_string()));
        assert_eq!("Assert".to_string(), decode("Atsrse".to_string()));
        assert_eq!("Hello world!".to_string(), decode("H!edlllroo w".to_string()));
        assert_eq!("You have chosen to translate this kata.".to_string(), decode("Y.oaut ahka vsei hcth oesteanl stnoa rt".to_string()));
    }
    
}
