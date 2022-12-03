// https://www.codewars.com/kata/576bb71bbbcf0951d5000044/solutions/rust

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    let mut positive_count = 0;
    let mut negative_sum = 0;

    if input.len() == 0 {
        return [].to_vec();
    }

    for &i in input.iter() {
        if i > 0 {
            positive_count += 1;
        } else {
            negative_sum += i;
        }
    }
    [positive_count, negative_sum].to_vec()
}

#[cfg(test)]
mod tests {
    use super::count_positives_sum_negatives;
        
    fn dotest(a: &[i32], expected: &[i32]) {
        let actual = count_positives_sum_negatives(a.to_vec());
        assert!(actual == expected, 
            "With input = {a:?}\nExpected {expected:?} but got {actual:?}")
    }
    #[test]
    fn fixed_tests() {
        dotest(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15], &[10, -65]);
        dotest(&[], &[]);
        dotest(&[0, 2, 3, 0, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14], &[8, -50]);
        dotest(&[0,1,2,3,4,5], &[5, 0]);
        dotest(&[1,2,3,4,5], &[5, 0]);
        dotest(&[0,-1,-2,-3,-4,-5], &[0, -15]);
        dotest(&[-1,-2,-3,-4,-5], &[0, -15]);
        dotest(&[0,0,0,0], &[0,0]);
        dotest(&[0], &[0,0]);
    }
}
