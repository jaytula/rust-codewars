// https://www.codewars.com/kata/535474308bb336c9980006f2/solutions/rust

fn greet(name: &str) -> String {
    format!("Hello {}{}!", &name[0..1].to_uppercase(), &name[1..].to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::greet;
    
    const ERR: &str = "\nYour result (left) did not match the expected output (right).";
    
    #[test]
    fn returns_expected() {
        assert_eq!(greet("riley"), "Hello Riley!", "{ERR}");
        assert_eq!(greet("JACK"), "Hello Jack!", "{ERR}");
    }   
}