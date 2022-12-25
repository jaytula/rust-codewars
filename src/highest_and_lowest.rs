// https://www.codewars.com/kata/554b4ac871d6813a03000035/solutions/rust

struct HighLow {
    pub high: i32,
    pub low: i32,
}

impl HighLow {
    fn new(numbers: &str) -> HighLow {
        let mut s = HighLow { high: 0, low: 0 };

        let numbers = numbers
            .split_whitespace()
            .map(|seg| seg.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        // println!("{:?}", numbers);
        s.high = *numbers.iter().max().unwrap();
        s.low = *numbers.iter().min().unwrap();
        s
    }
    fn to_string(self) -> String {
        format!("{} {}", self.high, self.low)
    }
}

fn high_and_low(numbers: &str) -> String {
    let s = HighLow::new(numbers);
    s.to_string()
}

#[test]
fn example_test_1() {
    assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}

#[test]
fn example_test_2() {
    assert_eq!("3 1", high_and_low("1 2 3"));
}
