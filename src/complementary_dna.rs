// https://www.codewars.com/kata/554e4a2f232cdd87d9000038/solutions/rust

fn dna_strand(dna: &str) -> String {
    let mut res = String::from("");
    for i in dna.chars() {
        match i {
            'A' => res.push('T'),
            'T' => res.push('A'),
            'G' => res.push('C'),
            'C' => res.push('G'),
            _ => panic!("Alien DNA")
        }
    }
    res
}

// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

#[cfg(test)]
mod tests {
    use super::dna_strand;

    fn dotest(s: &str, expected: &str) {
        let actual = dna_strand(s);
        assert!(actual == expected, 
            "With dna = \"{s}\"\nExpected \"{expected}\" but got \"{actual}\"")
    }
    
    #[test]
    fn fixed_tests() {
        dotest("AAAA","TTTT");
        dotest("ATTGC","TAACG");
        dotest("GTAT","CATA");
    }
}