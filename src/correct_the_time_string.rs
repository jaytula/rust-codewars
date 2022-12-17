// https://www.codewars.com/kata/57873ab5e55533a2890000c7/solutions/rust

use regex::Regex;

fn time_correct(time_str: &str) -> Option<String> {
    let re = Regex::new(r"^(\d{2}):(\d{2}):(\d{2})$").unwrap();

    if !re.is_match(time_str) {
        return None;
    }

    for cap in re.captures_iter(time_str) {
        let seconds = &cap[3].parse::<i32>().unwrap();
        let minutes = &cap[2].parse::<i32>().unwrap();
        let hours = &cap[1].parse::<i32>().unwrap();

        let res = format!(
            "{:02}:{:02}:{:02}",
            (hours + (minutes + (seconds / 60)) / 60) % 24,
            (minutes + (seconds / 60)) % 60,
            seconds % 60
        );
        return Some(res);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert!(time_correct("").is_none());
    }

    #[test]
    fn invalid_format() {
        assert!(time_correct("001122").is_none());
        assert!(time_correct("00;11;22").is_none());
        assert!(time_correct("00:1c:22").is_none());
    }

    #[test]
    fn corrections() {
        assert_eq!(time_correct("09:10:01"), Some(String::from("09:10:01")));
        assert_eq!(time_correct("11:70:10"), Some(String::from("12:10:10")));
        assert_eq!(time_correct("19:99:09"), Some(String::from("20:39:09")));
        assert_eq!(time_correct("19:99:99"), Some(String::from("20:40:39")));
        assert_eq!(time_correct("24:01:01"), Some(String::from("00:01:01")));
        assert_eq!(time_correct("52:01:01"), Some(String::from("04:01:01")));
    }
}
