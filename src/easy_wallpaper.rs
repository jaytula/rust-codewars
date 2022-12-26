// https://www.codewars.com/kata/567501aec64b81e252000003/solutions/rust

fn wall_paper(l: f64, w: f64, h: f64) -> String {
  if l == 0.0 || h == 0.0 || w == 0.0 { return "zero".to_string(); }
  let n = (((2.0 * (l + w) * h * 10000.0 * 1.15)) / (52.0 * 1000.0)).ceil() as usize;
  let numbers = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
    "twenty",
  ];
  numbers[n].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(l: f64, w: f64, h: f64, exp: &str) -> () {
        println!("l: {:?}", l);
        println!("w: {:?}", w);
        println!("h: {:?}", h);
        let ans = wall_paper(l, w, h);
        println!("actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }
    #[test]
    fn basic_tests() {
        dotest(6.3, 4.5, 3.29, "sixteen");
        dotest(6.3, 5.8, 3.13, "seventeen");  
        dotest(0.0, 5.4, 3.33, "zero");
    }

}
