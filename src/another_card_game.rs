pub fn the_game(frank: &[u8; 4], sam: &[u8; 4], tom: &[u8; 4]) -> bool {
    let res = true;

    let mut frank_wins = 0;
    let mut sam_wins: i32 = 0;
    let mut tom_wins = 0;

    let mut frank_left = frank.to_vec();
    let mut sam_left = sam.to_vec();
    let mut tom_left = tom.to_vec();

    if sam[0] == 0 {
        sam_left.remove(0);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_day() {
        assert_eq!(the_game(&[2, 5, 8, 11], &[1, 4, 7, 10], &[0, 3, 6, 9]), true);
    }
    
    #[test]
    fn bad_day() {
        assert_eq!(the_game(&[1, 2, 3, 4], &[5, 6, 7, 8], &[0, 9, 10, 11]), false);
    }


    #[test]
    fn random_day() {
        assert_eq!(the_game(&[1, 4, 5, 7], &[2, 9, 10, 11], &[0, 3, 6, 8]), false);
    }
}