// https://www.codewars.com/kata/63ab271e96a48e000e577442/solutions/rust

fn can_escape(walls: &[(usize, usize)]) -> bool {
    walls.iter().enumerate().fold(true, |acc, (idx, (wt, wb))| {
        acc && (*wt > idx + 1) && (*wb > idx + 1)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        for (case, expected) in [
            (&[(7, 7), (7, 7), (7, 7), (7, 7)][..], true),
            (&[(2, 2), (3, 3), (4, 4)], true),
            (&[(2, 8), (7, 3), (6, 4)], true),
            (&[(2, 2), (3, 3), (3, 4)], false),
            (&[(3, 3), (1, 1), (3, 3)], false),
            (&[(7, 7), (0, 0), (7, 7)], false),
        ] {
            assert_eq!(
                can_escape(case),
                expected,
                "Incorrect answer for walls {case:?}: should be {expected}"
            );
        }
    }
}
