// https://www.codewars.com/kata/563a631f7cbbc236cf0000c2/solutions/rust

fn move_hero(position: u32, roll: u32) -> u32 {
    position + roll * 2
}

#[cfg(test)]
mod tests {
    use super::move_hero;
    
    #[test]
    fn sample_tests() {
        assert_eq!(move_hero(0, 4), 8);
        assert_eq!(move_hero(3, 6), 15);
    }
}