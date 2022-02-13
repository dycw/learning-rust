// The Anatomy of a Test Function

#[cfg(test)]
mod tests_1 {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}

// Checking Results with the assert! Macro

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold_1(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests_2 {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold_1(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold_1(&larger));
    }
}

impl Rectangle {
    fn can_hold_2(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests_3 {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold_2(&smaller));
    }
}

// Testing Equality with the assert_eq! and assert_ne! Macros

pub fn add_two_1(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests_4 {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two_1(2))
    }
}

pub fn add_two_2(a: i32) -> i32 {
    a + 3
}

#[cfg(test)]
mod tests_5 {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two_2(2))
    }
}

// Adding Custom Failure Messages

pub fn greeting_1(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests_6 {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting_1("Carol");
        assert!(result.contains("Carol"));
    }
}

pub fn greeting_2(name: &str) -> String {
    String::from("Hello!")
}

#[cfg(test)]
mod tests_7 {
    use super::*;

    #[test]
    fn greeting_contains_name_1() {
        let result = greeting_2("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    fn greeting_contains_name_2() {
        let result = greeting_2("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}

// Checking for Panics with should_panic

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new_1(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests_8 {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new_1(200);
    }
}

impl Guess {
    pub fn new_2(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests_9 {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new_2(200);
    }
}

impl Guess {
    pub fn new_3(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests_10 {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new_3(200);
    }
}

impl Guess {
    pub fn new_4(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests_11 {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new_4(200);
    }
}

// Using Result<T, E> in Tests

#[cfg(test)]
mod tests_12 {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
