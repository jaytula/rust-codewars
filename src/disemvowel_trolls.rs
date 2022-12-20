// https://www.codewars.com/kata/52fba66badcd10859f00097e/solutions/rust

fn disemvowel(s: &str) -> String {
    let res = s
        .chars()
        .filter(|ch| !"aeiou".contains(ch.to_ascii_lowercase()))
        .collect();
    res
}

#[cfg(test)]
mod tests {
    use super::disemvowel;

    #[test]
    fn example_test() {
        assert_eq!(
            disemvowel("This website is for losers LOL!"),
            "Ths wbst s fr lsrs LL!"
        );
    }
}
