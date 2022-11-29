// fn to_csv_text(arr: &[Vec<i8>]) -> String {
//     return String::from("0,1,2,3,45\n10,11,12,13,14\n20,21,22,23,24\n30,31,32,33,34");
// }

fn join_with_comma(v: Vec<i8>) -> String {
    let mut vs = vec![];
    for i in v.iter() {
        println!("i: {}", i);
        vs.push(i.to_string());
    }
    let s = vs.join(",");
    return s;
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    // use super::to_csv_text;
    use super::join_with_comma;
    
    // fn do_test(input: &[Vec<i8>], expected: &str) {
    //     let actual  = to_csv_text(input);
    //     assert!(actual == expected,
    //         "Test failed with array = {input:?}\nExpected \"{expected}\"\nbut got \"{actual}\"");
    // }

    #[test]
    fn fixed_tests() {
        let actual = join_with_comma(vec![0, 1, 2, 3, 4, 5]);
        let expected = "0,1,2,3,4,5";
        assert!(actual == expected, "Yay");

        // for (input, expected) in [
        //     (vec![
        //     vec![0, 1, 2, 3, 45],
        //     vec![10, 11, 12, 13, 14],
        //     vec![20, 21, 22, 23, 24],
        //     vec![30, 31, 32, 33, 34]
        // ], "0,1,2,3,45\n10,11,12,13,14\n20,21,22,23,24\n30,31,32,33,34"),
        //     (vec![
        //     vec![-25, 21, 2, -33, 48],
        //     vec![30, 31, -32, 33, -34]
        // ], "-25,21,2,-33,48\n30,31,-32,33,-34"),
        //     (vec![
        //     vec![5, 55, 5, 5, 55],
        //     vec![6, 6, 66, 23, 24],
        //     vec![127, 31, 66, 33, 7]
        // ], "5,55,5,5,55\n6,6,66,23,24\n127,31,66,33,7")
        // ] {
        //     do_test(&input, expected)
        // }
    }
}
