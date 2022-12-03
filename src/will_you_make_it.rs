// https://www.codewars.com/kata/5861d28f124b35723e00005e/solutions/rust

fn zero_fuel(distance_to_pump: u32, mpg: u32, gallons: u32) -> bool {
    mpg * gallons >= distance_to_pump
}

#[cfg(test)]
mod tests {
    use super::zero_fuel;

    #[test]
    fn sample_tests() {
        assert_eq!(zero_fuel(50, 25, 2), true);
        assert_eq!(zero_fuel(100, 50, 1), false);
    }
}
