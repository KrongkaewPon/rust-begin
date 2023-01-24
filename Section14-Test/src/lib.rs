#![allow(dead_code)]
struct Rectangle {
    length: u32,
    width: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 5,
            width: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            length: 3,
            width: 3,
        };
        let smaller = Rectangle {
            length: 5,
            width: 4,
        };
        assert!(!larger.can_hold(&smaller));
    }

    #[test]
    fn it_add_two() {
        assert_ne!(add_two(3), 4);
    }
}
