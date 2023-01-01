// https://www.codewars.com/kata/5700c9acc1555755be00027e/solutions/rust

fn contain_all_rots(strng: &str, arr: Vec<&str>) -> bool { 
    for i in 0..strng.len() {
        let needle = format!("{}{}", &strng[i..(strng.len())], &strng[0..i]);
        if !arr.contains(&needle.as_str()) { return false; }
    }
    true
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(strng: &str, arr: Vec<&str>, exp: bool) -> () {
        println!("strng: {}", strng);
        println!("arr: {:?}", arr);
        let ans = contain_all_rots(strng, arr);
        println!("actual:\n{}", ans);
        println!("expect:\n{}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basis_tests() {
        dotest("", vec![], true);      
        dotest("bsjq", vec!["bsjq", "qbsj", "sjqb", "twZNsslC", "jqbs"], true);
        dotest("XjYABhR", vec!["TzYxlgfnhf", "yqVAuoLjMLy", "BhRXjYA", "YABhRXj", "hRXjYAB", "jYABhRX", "XjYABhR", "ABhRXjY"], false);
        
    }
}