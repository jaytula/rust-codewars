// https://www.codewars.com/kata/54fb963d3fe32351f2000102/solutions/rust

fn collatz(n: u64) -> u64 {
    let mut n = n;
    let mut vec = Vec::new();
    loop {
        vec.push(n);
        if n == 1 { return vec.len() as u64; }
        n = if n % 2 == 0 { n / 2 } else { (n * 3) + 1 };
    }
}

#[cfg(test)]
mod tests {
    use super::collatz;
        
    fn dotest(n: u64, expected: u64) {
        let actual = collatz(n);
        assert!(actual == expected, 
            "With n = {n}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(20, 8);
        dotest(15, 18);
    }
}
