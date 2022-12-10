// https://www.codewars.com/kata/5963c18ecb97be020b0000a2/solutions/rust

fn derive(coefficient: u32, exponent: u32) -> String {
    format!("{}x^{}", coefficient*exponent, exponent-1)
}

#[cfg(test)]
mod tests {
    use super::derive;

    #[test]
    fn test_basics() {
        assert_eq!(derive(7, 8), "56x^7");
        assert_eq!(derive(5, 9), "45x^8");
    }
}
