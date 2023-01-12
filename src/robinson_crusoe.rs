// https://www.codewars.com/kata/5d95b7644a336600271f52ba/train/rust

use std::f64;

fn crusoe(n: i32, d: f64, ang: i32, distmult: f64, angmult: f64) -> (f64, f64) {
    // your code
}

#[cfg(test)]
    mod tests {
    use super::*;
    use float_eq::float_eq;

    fn dotest(n: i32, d: f64, ang: i32, distmult: f64, angmult: f64, expect: (f64, f64)) -> () {
        let merr = 1.0e-9;
        let actual = crusoe(n, d, ang, distmult, angmult);
        let res0 = float_eq!(actual.0, expect.0, abs <= merr) || float_eq!(actual.0, expect.0, rmax <= merr);
        let res1 = float_eq!(actual.1, expect.1, abs <= merr) || float_eq!(actual.1, expect.1, rmax <= merr);
        assert!(res0, "Expected.0 value must be near: {:e} but was:{:e}", expect.0, actual.0);
        assert!(res1, "Expected.1 value must be near: {:e} but was:{:e}", expect.1, actual.1);
    }

    #[test]
    fn basic_tests() {
        dotest(8, 0.22, 3, 1.01, 1.15, (1.814652098870, 0.164646220964));
        dotest(29, 0.13, 21, 1.01, 1.09, (0.318341393410, 2.292862212314));
        dotest(45, 0.10, 3, 1.01, 1.10, (2.689897523779, 2.477953232467));
        
    }
}

