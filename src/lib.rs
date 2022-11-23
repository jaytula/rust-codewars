
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

mod invert_values;

#[cfg(test)]

fn minimum(arr: &[i32]) -> i32 {
    let mut min = i32::MAX;

    for i in 0..arr.len() {
        if arr[i] < min {
            min = arr[i];
        }
    }
    return min;
}

fn maximum(arr: &[i32]) -> i32 {
    let mut max = i32::MIN;

    for i in 0..arr.len() {
        if arr[i] > max {
            max = arr[i];
        }
    }
    return max;
}
mod tests {
    use super::{minimum, maximum};

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(arr: &[i32], expected_min: i32, expected_max: i32) {
        assert_eq!(minimum(arr), expected_min, "{ERR_MSG} with function minimum and arr = {arr:?}");
        assert_eq!(maximum(arr), expected_max, "{ERR_MSG} with function maximum and arr = {arr:?}");
    }

    #[test]
    fn fixed_tests() {
        dotest(&[-52, 56, 30, 29, -54, 0, -110], -110, 56);
        dotest(&[42, 54, 65, 87, 0], 0, 87);
        dotest(&[1, 2, 3, 4, 5, 10], 1, 10);
        dotest(&[-1, -2, -3, -4, -5, -10, 534, 43, 2, 1, 3, 4, 5, 5, 443, 443, 555, 555], -10, 555);
        dotest(&[9], 9, 9);
        dotest(&[4,6,2,1,9,63,-134,566], -134, 566);
    }
}

