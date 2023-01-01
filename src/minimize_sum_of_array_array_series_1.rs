// https://www.codewars.com/kata/5a523566b3bfa84c2e00010b/solutions/rust

fn min_sum(xs: &[u64]) -> u64 {
    let mut xs = xs.to_vec();
    let mut sum = 0u64;
    xs.sort();

    for i in 0..xs.len()/2 {
        sum += xs[i] * xs[xs.len() - 1 - i];
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(min_sum(&[5,4,2,3]), 22);
        assert_eq!(min_sum(&[12,6,10,26,3,24]), 342);
        assert_eq!(min_sum(&[9,2,8,7,5,4,0,6]), 74);        
    }        
}