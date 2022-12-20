// https://www.codewars.com/kata/52fba66badcd10859f00097e/solutions/rust

fn disemvowel(s: &str) -> String {
    s.replace("a", "")
    .replace("e", "")
    .replace("i", "")
    .replace("o", "")
    .replace("u", "")
    .replace("A", "")
    .replace("E", "")
    .replace("I", "")
    .replace("O", "")
    .replace("U", "")
}

#[cfg(test)]
mod tests {
    use super::disemvowel;
    
    #[test]
    fn example_test() {
        assert_eq!(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
    }
}