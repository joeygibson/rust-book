#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length &&
            self.width > other.width
    }
}

struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value,
        }
    }
}

pub fn add_two(n: u32) -> u32 {
    n + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn another() {
        panic!("this test fails");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller))
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(!smaller.can_hold(&larger))
    }

    #[test]
    fn test_messages() {
        let exp = 23;
        let res = 23;

        assert_eq!(exp, res,
                   "Wrong thing. Expected {}, got {}", exp, res);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn test_better_panic_check() {
        Guess::new(101);
    }
}
