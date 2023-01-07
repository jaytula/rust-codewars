// https://www.codewars.com/kata/5506b230a11c0aeab3000c1f/solutions/rust

fn evaporator(content: f64, evap_per_day: i32, threshold: i32) -> i32 {
    let mut res = 0i32;
    let mut current = content;
    let threshold = (threshold as f64) / 100.0 * content;

    loop {
        if current < threshold { return res; }
        current *= (100.0 - evap_per_day as f64) / 100.0;
        res += 1;
    }
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(_content: f64, evap_per_day: i32, threshold: i32, exp: i32) -> () {
        println!(" evap_per_day: {:?}", evap_per_day);
        println!("threshold: {:?}", threshold);
        let ans = evaporator(_content, evap_per_day, threshold);
        println!(" actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }
    
    #[test]
    fn basic_tests() {
        dotest(10.0, 10, 10, 22);
        dotest(10.0, 10, 5, 29);
        
    }
}