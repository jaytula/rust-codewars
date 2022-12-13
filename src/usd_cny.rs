// https://www.codewars.com/kata/5977618080ef220766000022/solutions/rust

fn usdcny(usd: u16) -> String {
    format!("{:.2} Chinese Yuan", (usd as f64) * 6.75)
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(usdcny(4), "27.00 Chinese Yuan");
        assert_eq!(usdcny(15), "101.25 Chinese Yuan");
        assert_eq!(usdcny(465), "3138.75 Chinese Yuan");
    }
}
