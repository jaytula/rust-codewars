// https://www.codewars.com/kata/5a905c2157c562994900009d/solutions/rust

use itertools::Itertools;

fn product_array(arr: &[u64]) -> Vec<u64> {
    arr.to_vec()
        .iter()
        .enumerate()
        .map(|(i, _)| {
            arr.iter().enumerate().fold(1u64, |acc, (idx, curr)| {
                acc * (if i == idx { 1u64 } else { *curr })
            })
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(product_array(&[12, 20]), [20, 12]);
        assert_eq!(product_array(&[3, 27, 4, 2]), [216, 24, 162, 324]);
        assert_eq!(
            product_array(&[13, 10, 5, 2, 9]),
            [900, 1170, 2340, 5850, 1300]
        );
        assert_eq!(
            product_array(&[16, 17, 4, 3, 5, 2]),
            [2040, 1920, 8160, 10880, 6528, 16320]
        );
    }
}
