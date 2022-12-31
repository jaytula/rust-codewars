// https://www.codewars.com/kata/57a1d5ef7cb1f3db590002af/solutions/rust

fn fib(n:u32)->u32{
    if n == 0 { return 0; }
    if n == 1 || n == 2 { return 1; }
    fib(n - 1) + fib(n - 2)
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::fib;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(n: u32, expected: u32) {
        assert_eq!(fib(n), expected, "{ERR_MSG} with n = {n}")
    }

    #[test]
    fn returns_expected() {
        dotest(0, 0);
        dotest(1, 1);
        dotest(2, 1);
        dotest(3, 2);
        dotest(4, 3);
        dotest(5, 5);
    }
}

