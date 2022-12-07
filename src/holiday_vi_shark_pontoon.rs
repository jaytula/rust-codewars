// https://www.codewars.com/kata/57e921d8b36340f1fd000059/solutions/rust

fn shark(
    pontoon_distance: f64,
    shark_distance: f64,
    you_speed: f64,
    shark_speed: f64,
    dolphin: bool,
) -> String {
    let shark_speed_adj = shark_speed / (1_f64 + ((dolphin as i8) as f64));
    let cond = pontoon_distance / you_speed <= shark_distance / shark_speed_adj;
    if cond {
        "Alive!".to_string()
    } else {
        "Shark Bait!".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(shark(12.0, 50.0, 4.0, 8.0, true), "Alive!");
        assert_eq!(shark(7.0, 55.0, 4.0, 16.0, true), "Alive!");
        assert_eq!(shark(24.0, 0.0, 4.0, 8.0, true), "Shark Bait!");
        assert_eq!(shark(40.0, 35.0, 3.0, 20.0, true), "Shark Bait!");
        assert_eq!(shark(7.0, 8.0, 3.0, 4.0, true), "Alive!");
    }
}
