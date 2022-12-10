// https://www.codewars.com/kata/57cfdf34902f6ba3d300001e/solutions/rust

fn two_sort(arr: &[&str]) -> String {
    let mut s = String::from(arr[0]);

    for &i in arr.iter() {
        if String::from(i) < s {
            s = String::from(i);
        }
    }
    
    let mut y = String::from("");

    for (i, ch) in s.chars().enumerate() {
        if i != 0 { y.push_str("***"); }
        y.push(ch);
    }
    y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test_cases() {
        // assert_eq!(
        //     two_sort(&[
        //         "turns", "out", "random", "test", "cases", "are", "easier", "than", "writing",
        //         "out", "basic", "ones"
        //     ]),
        //     "are"
        // );
        // assert_eq!(
        //     two_sort(&[
        //         "bitcoin", "take", "over", "the", "world", "maybe", "who", "knows", "perhaps"
        //     ]),
        //     "b***i***t***c***o***i***n"
        // );
        assert_eq!(
            two_sort(&[
                "turns", "out", "random", "test", "cases", "are", "easier", "than", "writing",
                "out", "basic", "ones"
            ]),
            "a***r***e"
        );
    }
}
