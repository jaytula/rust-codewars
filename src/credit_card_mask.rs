// https://www.codewars.com/kata/5412509bd436bd33920011bc/solutions/rust

/// Return a String with all characters masked as '#' except the last 4.
fn maskify(cc: &str) -> String {
    if cc.len() == 0 { return String::from(""); }
    let num_hashes = if cc.len() > 4  {cc.len() - 4 } else { 0 };
    let end_index = if cc.len() > 4 {cc.len() - 4} else { 0 };

    format!("{}{}", "#".repeat(num_hashes), cc.chars().skip(end_index).collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::maskify;

    #[test]
    fn it_masks_example_strings() {
        assert_eq!(maskify("4556364607935616"), "############5616");
        assert_eq!(maskify("1"), "1");
        assert_eq!(maskify(""), "");
        assert_eq!(maskify("11111"), "#1111");
        assert_eq!(maskify("BUM"), "BUM");
    }
}