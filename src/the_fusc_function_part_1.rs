// https://www.codewars.com/kata/570409d3d80ec699af001bf9/solutions/rust

fn fusc(n: u32) -> u32{
    if n == 0 { return 0; }
    if n == 2 { return fusc(1); }
    if n == 1 { return 1; }
    if n % 2 == 0 { return fusc(n / 2); }
    fusc(n / 2) + fusc(n / 2 + 1)
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::fusc;

    #[test]
    fn tests() {
        assert_eq!(fusc(0), 0, "\nWith n=0, your answer (left) is not the expected answer (right).");
        assert_eq!(fusc(1), 1, "\nWith n=1, your answer (left) is not the expected answer (right).");
        assert_eq!(fusc(85), 21, "\nWith n=85, your answer (left) is not the expected answer (right).");
    }
}
