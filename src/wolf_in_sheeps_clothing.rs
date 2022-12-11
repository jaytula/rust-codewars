// https://www.codewars.com/kata/5c8bfa44b9d1192e1ebd3d15/solutions/rust

fn warn_the_sheep(queue: &[&str]) -> String {
    let mut position: usize = 0;

    for (i, &s) in queue.iter().rev().enumerate() {
        if s == "wolf" {
            position = i;
            break;
        }
    }

    if position == 0 { return "Pls go away and stop eating my sheep".to_string(); }
    format!("Oi! Sheep number {}! You are about to be eaten by a wolf!", position)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            warn_the_sheep(&["sheep", "sheep", "sheep", "sheep", "sheep", "wolf", "sheep", "sheep"]),
            "Oi! Sheep number 2! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["sheep", "wolf", "sheep", "sheep", "sheep", "sheep", "sheep"]),
            "Oi! Sheep number 5! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["wolf", "sheep", "sheep", "sheep", "sheep", "sheep", "sheep"]),
            "Oi! Sheep number 6! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["sheep", "wolf", "sheep"]),
            "Oi! Sheep number 1! You are about to be eaten by a wolf!",
        );
        assert_eq!(
            warn_the_sheep(&["sheep", "sheep", "wolf"]),
            "Pls go away and stop eating my sheep",
        );
    }
}
