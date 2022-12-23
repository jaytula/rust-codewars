use itertools::Itertools;

pub fn the_game(frank: &[u8; 4], sam: &[u8; 4], tom: &[u8; 4]) -> bool {
    let res = true;

    let mut frank_wins = 0;
    let mut sam_wins: i32 = 0;
    let mut tom_wins = 0;

    let mut frank_left = frank.to_vec();
    let mut sam_left = sam.to_vec();
    let mut tom_left = tom.to_vec();

    println!("Round 0: {:?} {:?} {:?}", frank_left, sam_left, tom_left);

    if sam[0] == 0 {
        sam_left.remove(0);
        let tom_round1 = tom_left[1];
        let frank_round1_index = frank_left.iter().find_position(|&el| *el > tom_round1);
        if frank_round1_index == None { return false; }
        let (to_remove, _) = frank_round1_index.unwrap();
        frank_left.remove(to_remove);
        tom_left.remove(1);
        frank_wins += 1;       
    }

    if tom[0] == 0 {
        tom_left.remove(0);
        let sam_round1 = sam_left[1];
        let frank_round1_index = frank_left.iter().find_position(|&el| *el > sam_round1);
        if frank_round1_index == None { return false; }
        let (to_remove, _) = frank_round1_index.unwrap();
        frank_left.remove(to_remove);
        sam_left.remove(1);
        frank_wins += 1;       
    }

    if frank[0] == 0 {
        frank_left.remove(0);
        if sam_left[3] > tom_left[3] {
            sam_wins = 1;
        } else {
            tom_wins = 1;
        }
        sam_left.remove(3);
        tom_left.remove(3);
    }

    println!("Round 1: {:?} {:?} {:?}", frank_left, sam_left, tom_left);
    println!("Round 1 Win: {} {} {}", frank_wins, sam_wins, tom_wins);

    if frank_wins == 1 {
        if frank[2] > sam[0] || frank[2] > tom[0] { return true; }

    }
    if frank[2] < sam[1] || frank[2] < tom[1] { return false; }
    if frank[1] < sam[0] || frank[1] < tom[0] { return false; }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weird_day() {
        assert_eq!(the_game(&[0, 6, 7, 8], &[1, 2, 3, 11], &[4, 5, 9, 10]), true);
    }

    #[test]
    fn weird_day_2() {
        assert_eq!(the_game(&[4, 5, 6, 9], &[1, 2, 3, 7], &[0, 8, 10, 11]), true)
    }

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