// https://www.codewars.com/kata/571640812ad763313600132b/solutions/rust

fn alex_mistakes(number_of_katas: u32, time_limit: u32) -> u32 {
    let mut mistakes = 0;
    let remaining = time_limit - number_of_katas * 6 ;
    
    loop {
      if 10 * 2u32.pow(mistakes) - 5 > remaining { break; }
      mistakes += 1;
    }
    mistakes
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::alex_mistakes;
        
    fn dotest(k: u32, t:u32, expected: u32) {
        let actual = alex_mistakes(k, t);
        assert!(actual == expected, "With number_of_katas = {k}, time_limit = {t}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(10, 120, 3);
        dotest(11, 120, 3);
        dotest(3, 45, 2);
        dotest(8, 120, 3);
        dotest(6, 60, 2);
        dotest(9, 180, 4);
        dotest(20, 120, 0);
        dotest(20, 125, 1);
        dotest(20, 130, 1);
        dotest(20, 135, 2);
    }
}
