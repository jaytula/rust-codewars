// https://www.codewars.com/kata/5656b6906de340bd1b0000ac/solutions/rust

fn longest(a1: &str, a2: &str) -> String {
    let mut res = String::from("");
    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        if !a1.contains(c) && !a2.contains(c) { continue; }
        res.push(c);
    }
    res
}

#[cfg(test)]
    mod tests {
    use super::*;
   
    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", longest(s1, s2), exp);
        println!("{}", longest(s1, s2) == exp);
        assert_eq!(&longest(s1, s2), exp)
    }

    #[test]
    fn basic_tests() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
        testing("loopingisfunbutdangerous", "lessdangerousthancoding", "abcdefghilnoprstu");
        
    }
}
