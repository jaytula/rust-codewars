struct Quark {
    color: String,
    flavor: String,
}

impl Quark {
    fn new(color: &str, flavor: &str) -> Quark {
        Quark {
            color: color.to_string(),
            flavor: flavor.to_string()
        }
    }

    fn color(&self) -> &str {
        &self.color
    }

    fn flavor(&self) -> &str {
        &self.flavor
    }

    fn baryon_number(&self) -> f32 {
        1.0 / 3.0
    }

    fn interact(&mut self, other: &mut Quark ) {
        std::mem::swap(&mut self.color, &mut other.color);
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_initialization_and_attribute() {
        let q1 = Quark::new("red", "up");
        let q2 = Quark::new("blue", "strange");
        assert!(q1.color() == "red", "q1.color() should return \"red\" but got\"{}\"", q1.color());
        assert!(q2.flavor() == "strange", "q2.flavor() should return \"strange\" but got \"{}\"", q2.flavor());
        assert!(q2.baryon_number() == 1.0 / 3.0, "q2.baryon_number() should return {} but got {}", 1.0 / 3.0, q2.baryon_number());
    }
    
    #[test]
    fn test_interaction() {
        let mut q1 = Quark::new("red", "up");
        let mut q2 = Quark::new("blue", "strange");
        q1.interact(&mut q2);
        assert!(q1.color() == "blue", "After interaction with q2, q1.color() should return \"blue\" but got\"{}\"", q1.color());
        assert!(q2.color() == "red", "After interaction with q1, q2.color() should return \"red\" but got\"{}\"", q2.color());
    }
}
