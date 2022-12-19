// https://www.codewars.com/kata/61123a6f2446320021db987d/solutions/rust

fn prev_mult_of_three(n: i32) -> i32 {
    let n = n.to_string();
    for i in 0..n.len() {
        let num = &n[0..(n.len()-i)].parse::<i32>().unwrap();

        if num % 3 == 0 {
            return *num;
        }
    }
    -1
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(prev_mult_of_three(1), -1);
        assert_eq!(prev_mult_of_three(25), -1);
        assert_eq!(prev_mult_of_three(36), 36);
        assert_eq!(prev_mult_of_three(1244), 12);
        assert_eq!(prev_mult_of_three(952406), 9);
    }
}
