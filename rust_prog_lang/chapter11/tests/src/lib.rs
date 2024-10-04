pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(num: u64) -> u64 {
    num + 2
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod add_tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_three_and_two() {
        let result = add(3, 2);
        assert_eq!(result, 5);
    }

    #[test]
    fn add_twenty_and_two() {
        let result = add(20, 2);
        assert_eq!(result, 22);
    }

    #[test]
    fn adding_two_works() {
        assert_eq!(add_two(2), 4);
    }
}

#[cfg(test)]
mod rectangle_tests {
    use super::*;

    #[test]
    fn can_hold_smaller() {
        let r1 = Rectangle {
            width: 10,
            height: 10,
        };
        let r2 = Rectangle {
            width: 8,
            height: 7,
        };

        assert!(r1.can_hold(&r2));
    }

    #[test]
    fn cant_hold_larger() {
        let r1 = Rectangle {
            width: 10,
            height: 10,
        };
        let r2 = Rectangle {
            width: 8,
            height: 7,
        };

        assert!(!r2.can_hold(&r1));
    }
}
