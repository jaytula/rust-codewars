// https://www.codewars.com/kata/6167e70fc9bd9b00565ffa4e/solutions/rust

fn barista(coffees: &[u8]) -> u16 {
    let mut coffees = coffees.iter().map(|&el| el as u16).collect::<Vec<_>>();
    coffees.sort();

    let length = coffees.len() as u16;

    let mut total: u16 = 0;
    for (idx, &minutes) in coffees.iter().enumerate() {
        total += (length-idx as u16) * minutes + 2 * idx as u16;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::barista;

    #[test]
    fn sample_tests() {
        assert_eq!(barista(&[]), 0);
        assert_eq!(barista(&[5]), 5);
        assert_eq!(barista(&[5,6]), 10+6+2);

        assert_eq!(barista(&[2, 10, 5, 3, 9]), 85);
        assert_eq!(barista(&[4, 3, 2]), 22);
        assert_eq!(barista(&[20, 5]), 32);
        assert_eq!(barista(&[20, 5, 4, 3, 1, 5, 7, 8]), 211);
        assert_eq!(barista(&[5, 4, 3, 2, 1]), 55);
    }
}