// https://www.codewars.com/kata/586c1cf4b98de0399300001d/solutions/rust

fn combat(health: f32, damage: f32) -> f32 {
    let remaining = health - damage;
    if remaining < 0.0 { return 0.0; }
    remaining
}

#[cfg(test)]
mod tests {
    use super::combat;
    
    #[test]
    fn example_tests() {
        assert_eq!(combat(100.0, 5.0), 95.0);
        assert_eq!(combat(92.0, 8.0), 84.0);
        assert_eq!(combat(20.0, 30.0), 0.0, "Health cannot go below 0");
    }
}