// https://www.codewars.com/kata/55b75fcf67e558d3750000a3/solutions/rust

struct Block {
    width: u32,
    length: u32,
    height: u32,
}

impl Block {
    fn new(arr: &[u32]) -> Block {
        Block {
            width: arr[0],
            length: arr[1],
            height: arr[2],
        }
    }

    fn get_width(&self) -> u32 {
        return self.width;
    }

    fn get_length(&self) -> u32 {
        return self.length;
    }

    fn get_height(&self) -> u32 {
        return self.height;
    }

    fn get_volume(&self) -> u32 {
        return self.width * self.height * self.length;
    }

    fn get_surface_area(&self) -> u32 {
        return 2 * self.width * self.length
            + 2 * self.height * self.width
            + 2 * self.length * self.height;
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::Block;

    #[test]
    fn example_test() {
        let block = Block::new(&[2, 4, 6]);
        assert_eq!(
            block.get_width(),
            2,
            "Incorrect width\nExpected 2 but got {}",
            block.get_width()
        );
        assert_eq!(
            block.get_length(),
            4,
            "Incorrect length\nExpected 4 but got {}",
            block.get_length()
        );
        assert_eq!(
            block.get_height(),
            6,
            "Incorrect height\nExpected 6 but got {}",
            block.get_height()
        );
        assert_eq!(
            block.get_volume(),
            48,
            "Incorrect volume\nExpected 48 but got {}",
            block.get_volume()
        );
        assert_eq!(
            block.get_surface_area(),
            88,
            "Incorrect surface area\nExpected 88 but got {}",
            block.get_surface_area()
        );
    }
}
