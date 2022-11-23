fn invert(values: &[i32]) -> Vec<i32> {
    let mut vec = Vec::new();
    for i in 0..values.len() {
        vec.push(-values[i]);
    }
    return vec;
}

#[cfg(test)]
mod tests {
    use super::invert;

    #[test]
    fn basic_tests() {
        assert_eq!(invert(&vec![1,2,3,4,5]), vec![-1,-2,-3,-4,-5]);
        assert_eq!(invert(&vec![1,-2,3,-4,5]), vec![-1,2,-3,4,-5]);
        assert_eq!(invert(&vec![]), vec![]);
    }
}