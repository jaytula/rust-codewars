// https://www.codewars.com/kata/559cc2d2b802a5c94700000c/solutions/rust

fn consecutive(xs: &[i16]) -> i16 {
    if xs.len() == 0 {
        return 0;
    }
    let max = xs.iter().max().unwrap();
    let min = xs.iter().min().unwrap();

    max - min - xs.len() as i16 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(consecutive(&[4, 8, 6]), 2);
        assert_eq!(consecutive(&[1, 2, 3, 4]), 0);
        assert_eq!(consecutive(&[]), 0);
        assert_eq!(consecutive(&[1]), 0);
    }
}
