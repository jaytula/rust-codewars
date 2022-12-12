// https://www.codewars.com/kata/545af3d185166a3dec001190/solutions/rust

fn each_cons(arr: &[u8], n: usize) -> Vec<Vec<u8>> {
    let mut res = Vec::new();

    if n > arr.len() { return res; }

    for i in 0..=(arr.len()-n) {
        let sub = arr[i..i+n].to_vec();
        res.push(sub);
    }

    res
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::each_cons;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(arr: &[u8], n: usize, expected: Vec<Vec<u8>>) {
        assert_eq!(each_cons(arr, n), expected, "{ERR_MSG} with arr = {arr:?}, n = {n}")   
    }

    #[test]
    fn sample_tests() {
        let arr = &[3, 5, 8, 13];
        let empty_vec: Vec<Vec<u8>> = Vec::new();
        dotest(arr, 1, vec![vec![3], vec![5], vec![8], vec![13]]);
        dotest(arr, 2, vec![vec![3,5], vec![5,8], vec![8,13]]);
        dotest(arr, 3, vec![vec![3,5,8], vec![5,8,13]]);
        dotest(&vec![], 3,  empty_vec);
    }
}
