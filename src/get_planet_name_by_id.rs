// https://www.codewars.com/kata/515e188a311df01cba000003/solutions/rust

fn get_planet_name(id: u32) -> String {
    match id {
        1 => "Mercury".to_string(),
        2 => "Venus".to_string(),
        3 => "Earth".to_string(),
        4 => "Mars".to_string(),
        5 => "Jupiter".to_string(),
        6 => "Saturn".to_string(),
        7 => "Uranus".to_string(),
        8 => "Neptune".to_string(),
        _ => unreachable!(),
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_planet_name() {
        assert_eq!(get_planet_name(3), "Earth");
    }
}
