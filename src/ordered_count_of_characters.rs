use std::collections::HashMap;

fn ordered_count(sip: &str) -> Vec<(char, i32)> {
    let mut hs = HashMap::new();
    let mut keys = Vec::new();
    let mut vec: Vec<(char, i32)> = Vec::new();

    for ch in sip.chars() {
        if !hs.contains_key(&ch) { keys.push(ch); }
        *hs.entry(ch).or_insert(0) += 1;
    }

    for &ch in keys.iter() {
        vec.push((ch, hs[&ch]));
    }

    vec
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abradacadabra() {
        assert_eq!(
            ordered_count("abracadabra"),
            vec![('a', 5), ('b', 2), ('r', 2), ('c', 1), ('d', 1)]
        );
    }
    #[test]
    fn test_banana() {
        assert_eq!(ordered_count("banana"), vec![('b', 1), ('a', 3), ('n', 2)]);
    }
    #[test]
    fn test_master_solver() {
        assert_eq!(
            ordered_count("i am a master kata solver"),
            vec![
                ('i', 1),
                (' ', 5),
                ('a', 5),
                ('m', 2),
                ('s', 2),
                ('t', 2),
                ('e', 2),
                ('r', 2),
                ('k', 1),
                ('o', 1),
                ('l', 1),
                ('v', 1)
            ]
        );
    }
}
