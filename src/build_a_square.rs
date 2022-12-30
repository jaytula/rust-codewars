// https://www.codewars.com/kata/59a96d71dbe3b06c0200009c/solutions/rust

fn generate_shape(n: i32) -> String {
    let mut s = String::new();

    for i in 0..n {
        for _ in 0..n {
            s += "+";
        }
        if i == n-1 { break; }
        s += "\n";
    }
    s
}

#[cfg(test)]
mod tests {
    use super::generate_shape;
        
    fn dotest(n: i32, expected: &str) {
        let actual = generate_shape(n);
        assert!(actual == expected, 
            "With n = {n}\nExpected \"{expected}\" but got \"{actual}\"")
    }
    
    // fn reference_solution(n: i32) -> String {
    //     (0 .. n)
    //     .map(|_x| "+".repeat(n as usize)
    //     )
    //     .collect::<Vec<String>>().join("\n")
    // }
    
    #[test]
    fn sample_test() {
      dotest(3, "+++\n+++\n+++")
    }
}
