use itertools::Itertools;

fn find_lowest_int(k: u64) -> u64 {
    for n in 1..=u64::MAX {
        let s1: String = (n * k).to_string().chars().sorted().collect();
        let s2: String = (n * (k+1)).to_string().chars().sorted().collect();

        if s1 == s2 { return n; }
    }

    panic!("Not found!")
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::find_lowest_int;
        
    fn dotest(n: u64, expected: u64) {
        let actual = find_lowest_int(n);
        assert!(actual == expected, 
            "With k = {n}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(325,477);
        dotest(599,2394);
        dotest(855, 999);
        dotest(1,125874);
        dotest(100,8919);
        dotest(1000,89919);
        dotest(10000,899919);
    }
}
