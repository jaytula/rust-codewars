fn pofi(n: u32) -> &'static str {
    ["1", "i", "-1", "-i"][n as usize % 4]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        do_test(0, "1");
        do_test(1, "i");
        do_test(2, "-1");
        do_test(3, "-i");
        do_test(4, "1");
        do_test(5, "i");
        do_test(6, "-1");
        do_test(7, "-i");
        do_test(8, "1");
        do_test(9, "i");
        do_test(10, "-1");
    }
    
    fn do_test(inp: u32, exp: &'static str) {
        assert_eq!(pofi(inp), exp, "\nFailed with n: {}", inp);
    }
}
