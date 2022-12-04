// https://www.codewars.com/kata/5b73fe9fb3d9776fbf00009e/solutions/rust

fn sum_of_differences(arr: &[i8]) -> Option<i8> {
    if arr.len() <= 1 { return None; }
    let mut sum = 0 as i32;
    let mut vec = arr.to_vec();

    vec.sort();
    vec.reverse();

    for i in 1..vec.len() {
        sum += (vec[i] - vec[i-1]).abs() as i32;
    }
    return Some(sum as i8);
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::sum_of_differences;

    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    #[test]
    fn sample_tests() {
        assert_eq!(sum_of_differences(&[1, 2, 10]), Some(9), "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[-3, -2, -1]), Some(2), "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[1, 1, 1, 1, 1]), Some(0), "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[-17, 17]), Some(34), "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[]), None, "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[0]), None, "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[-1]), None, "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[1]), None, "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[9, 7, -15]), Some(24), "{}", ERR_MSG);
        assert_eq!(sum_of_differences(&[-19, -12, 12, -14, -17, 4, 5, -12, 18]), Some(37), "{}", ERR_MSG);

    }
}
