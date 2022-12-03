// https://www.codewars.com/kata/56200d610758762fb0000002/solutions/rust

fn add_five(num: i32) -> i32 {
    let total = num + 5;
    total
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        assert_eq!(add_five(5), 10);
        assert_eq!(add_five(0), 5);
        assert_eq!(add_five(-5), 0);
    }
}