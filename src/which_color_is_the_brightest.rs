// https://www.codewars.com/kata/62eb800ba29959001c07dfee/solutions/rust
use itertools::Itertools;

fn brightest(colors: &[String]) -> String {
    let res = colors.into_iter().map(|s| {
        [&s[1..3], &s[3..5], &s[5..7]]
            .iter()
            .map(|&el| u32::from_str_radix(el, 16).unwrap())
            .max().unwrap()
    }).collect_vec();

    let max = res.iter().max().unwrap();
    let index = res.iter().position(|x| x == max).unwrap();

    colors[index].to_string()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::brightest;

    fn dotest(colors: &[String], expected: &str) {
        let actual = brightest(colors);
        assert!(
            actual == expected,
            "With colors = {colors:?}\nExpected \"{expected}\" but got \"{actual}\""
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(
            &[String::from("#001000"), String::from("#000000")],
            "#001000",
        );
        dotest(
            &[String::from("#ABCDEF"), String::from("#123456")],
            "#ABCDEF",
        );
        dotest(
            &[String::from("#00FF00"), String::from("#FFFF00")],
            "#00FF00",
        );
        dotest(
            &[String::from("#FFFFFF"), String::from("#1234FF")],
            "#FFFFFF",
        );
        dotest(
            &[
                String::from("#FFFFFF"),
                String::from("#123456"),
                String::from("#000000"),
            ],
            "#FFFFFF",
        );
    }
}
