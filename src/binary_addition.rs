// https://www.codewars.com/kata/551f37452ff852b7bd000139/solutions/rust

fn add_binary(a: u64, b: u64) -> String {
    format!("{:b}", a+b)
}

#[cfg(test)]
mod tests {
    use super::add_binary;
    
    fn dotest(a: u64, b: u64, expected: &str) {
        let actual = add_binary(a, b);
        assert!(actual == expected, 
            "With a = {a}, b = {b}\nExpected \"{expected}\" but got \"{actual}\"")
    }

    #[test]
    fn sample_tests() {
        dotest(1,1,"10");
        dotest(0,1,"1");
        dotest(1,0,"1");
        dotest(2,2,"100");
        dotest(51,12,"111111");
    }
}