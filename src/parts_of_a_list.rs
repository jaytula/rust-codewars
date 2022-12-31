// https://www.codewars.com/kata/56f3a1e899b386da78000732/solutions/rust

fn part_list(arr: Vec<&str>) -> String {
    let res = (1..arr.len())
        .map(|num| format!("({}, {})", arr[0..num].join(" "), arr[num..].join(" ")))
        .collect::<Vec<_>>()
        .join("");
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(arr: Vec<&str>, exp: &str) -> () {
        println!("arr: {:?}", arr);
        let ans = part_list(arr);
        println!("actual:\n{}", ans);
        println!("expect:\n{}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basis_tests() {
        dotest(vec!["I", "wish", "I", "hadn't", "come"],
                "(I, wish I hadn't come)(I wish, I hadn't come)(I wish I, hadn't come)(I wish I hadn't, come)");
        dotest(
            vec!["cdIw", "tzIy", "xDu", "rThG"],
            "(cdIw, tzIy xDu rThG)(cdIw tzIy, xDu rThG)(cdIw tzIy xDu, rThG)",
        );
    }
}
