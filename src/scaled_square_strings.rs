// https://www.codewars.com/kata/56ed20a2c4e5d69155000301/solutions

use itertools::Itertools;

fn scale(s: &str, k: u32, n: u32) -> String {
    s.split_whitespace().map(|t| {
        t.chars()
            .map(|ch| ch.to_string().repeat(k as usize))
            .join("")
    }).map(|line| {
        let mut t = String::from("");
        for i in 0..n {
            t += &line;
            if i != n-1 {
                t += "\n";
            }
        }
        t
    }).join("\n")
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::scale;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest<S>(s: &str, k: u32, n: u32, expected: S)
    where
        S: AsRef<str> + std::cmp::Ord + std::fmt::Debug,
        std::string::String: std::cmp::PartialEq<S>,
    {
        assert_eq!(
            scale(s, k, n),
            expected,
            "{ERR_MSG} with s = \n\"{s}\",\nk = {k}, n = {n}"
        )
    }

    #[test]
    fn sample_tests() {
        dotest("abcd\nefgh\nijkl\nmnop", 2, 3, "aabbccdd\naabbccdd\naabbccdd\neeffgghh\neeffgghh\neeffgghh\niijjkkll\niijjkkll\niijjkkll\nmmnnoopp\nmmnnoopp\nmmnnoopp");
        dotest("", 5, 5, "");
        dotest("Kj\nSH", 1, 2, "Kj\nKj\nSH\nSH");
    }
}
