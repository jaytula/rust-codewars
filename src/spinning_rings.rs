// https://www.codewars.com/kata/59afff65f1c8274f270020f5/solutions/rust

fn spinning_rings (inner_max: u8, outer_max: u8) -> u8 {
    let mut count: u8 = 1;
    let mut inner: u8 = inner_max;
    let mut outer: u8 = 1;

    loop {
        // println!("{} {} {}", count, inner, outer);
        if inner == outer { return count; }
        count += 1;
        inner = if inner == 0 { inner_max } else { inner- 1};
        outer = (outer + 1) % (outer_max + 1);
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::spinning_rings;

    #[test]
    fn fixed_tests() {
        assert_eq!(spinning_rings(2, 3), 5);
        assert_eq!(spinning_rings(3, 2), 2);
        assert_eq!(spinning_rings(1, 1), 1);
        assert_eq!(spinning_rings(2, 2), 3);
        assert_eq!(spinning_rings(3, 3), 2);
    }
}