// https://www.codewars.com/kata/57aa218e72292d98d500240f/solutions/rust

fn heron(a: f64, b: f64, c: f64) -> f64 {
    let s = (a + b + c) / 2.0;

    (s * (s - a) * (s - b) * (s - c)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::heron;
    
    const EPSILON: f64 = 0.00001;
    
    #[test]
    fn sample_tests() {
        assertion(6.0, (3.0, 4.0, 5.0));
        assertion(24.0, (6.0, 8.0, 10.0));
    }
    
    fn assertion(expected: f64, inputs: (f64, f64, f64)) {
        let actual = heron(inputs.0, inputs.1, inputs.2);
        assert!((expected - actual).abs() < EPSILON,
                "expected around: {} \n\nactual: {} \n\ninputs: {:?}\n\n", expected, actual, inputs);
    }
}
