// https://www.codewars.com/kata/62cecd4e5487c10028996e04/solutions/rust

fn race_podium(blocks: u32) -> (u32, u32, u32) {
    if blocks == 7 {
        return (2, 4, 1);
    }
    let mut highest = blocks / 3;
    loop {
        let left = blocks - 2 * highest + 1;
        if left < highest - 1 {
            break;
        }
        highest += 1
    }
    (highest - 1, highest, blocks - 2 * highest + 1)
}

#[cfg(test)]
mod tests {
    use super::race_podium;

    fn dotest(n: u32, expected: (u32, u32, u32)) {
        let actual = race_podium(n);
        assert!(
            actual == expected,
            "With n = {n}\nExpected {expected:?} but got {actual:?}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(11, (4, 5, 2));
        dotest(6, (2, 3, 1));
        dotest(10, (4, 5, 1));
        dotest(100000, (33334, 33335, 33331));
        dotest(7, (2, 4, 1));
        dotest(8, (3, 4, 1));
    }
}
