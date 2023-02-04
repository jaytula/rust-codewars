// https://www.codewars.com/kata/57e8fba2f11c647abc000944/solutions/rust

use itertools::Itertools;

fn fire_fight(s: &str) -> String {
    s.split_ascii_whitespace()
        .map(|el| if el == "Fire" { "~~" } else { el })
        .join(" ")
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            fire_fight("Boat Rudder Mast Boat Hull Water Fire Boat Deck Hull Fire Propeller Deck Fire Deck Boat Mast"),
            "Boat Rudder Mast Boat Hull Water ~~ Boat Deck Hull ~~ Propeller Deck ~~ Deck Boat Mast");

        assert_eq!(
            fire_fight("Mast Deck Engine Water Fire"),
            "Mast Deck Engine Water ~~"
        );

        assert_eq!(
            fire_fight(
                "Fire Deck Engine Sail Deck Fire Fire Fire Rudder Fire Boat Fire Fire Captain"
            ),
            "~~ Deck Engine Sail Deck ~~ ~~ ~~ Rudder ~~ Boat ~~ ~~ Captain"
        );
    }
}
