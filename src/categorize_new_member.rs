// https://www.codewars.com/kata/5502c9e7b3216ec63c0001aa/solutions/rust

fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    data
        .iter()
        .map(|item| {
            let (a, b) = item;
            if *a >= 55 && *b > 7 {
                "Senior".to_string()
            } else {
                "Open".to_string()
            }
        })
        .collect::<Vec<_>>()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(
            open_or_senior(vec![(45, 12), (55, 21), (19, -2), (104, 20)]),
            vec!["Open", "Senior", "Open", "Senior"]
        );
        assert_eq!(
            open_or_senior(vec![(3, 12), (55, 1), (91, -2), (54, 23)]),
            vec!["Open", "Open", "Open", "Open"]
        );
    }
}
