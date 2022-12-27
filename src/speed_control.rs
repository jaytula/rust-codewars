// https://www.codewars.com/kata/56484848ba95170a8000004d/solutions/rust

fn gps(s: i32, x: Vec<f64>) -> i32 {
    if x.len() <= 1 { return 0; }
    let mut vec = Vec::new();
    for i in 1..x.len() {
        vec.push((((x[i] - x[i-1]) * 3600.0) / s as f64) as i32);
    }

    println!("vec: {:?}", vec);
    *vec.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(s: i32, x: Vec<f64>, exp: i32) -> () {
        println!("s: {:?};", s);
        println!("x: {:?};", x);
        let ans = gps(s, x);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        let mut x = vec![0.0, 0.23, 0.46, 0.69, 0.92, 1.15, 1.38, 1.61];
        let mut s = 20;
        let mut u = 41;
        dotest(s, x, u);
        x = vec![0.0, 0.11, 0.22, 0.33, 0.44, 0.65, 1.08, 1.26, 1.68, 1.89, 2.1, 2.31, 2.52, 3.25];
        s = 12;
        u = 219;
        dotest(s, x, u);
        
    }

    #[test]
    fn basic_tests_2() {
        let  x = vec![0.0, 0.02, 0.36, 0.54, 0.72, 0.9, 1.08, 1.26, 1.44, 1.62, 1.8];
        let  s = 17;
        let  u = 72;
        dotest(s, x, u);        
    }
}
