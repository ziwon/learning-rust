#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 5
}

pub fn greeting(name: &str) -> String {
    format!("Hello")
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        //if value < 1 {
        //panic!("Guess value must be greater than or equal to 1, got {}.",
        //value);
        //} else if value > 100 {
        //panic!("Guess value must be less than or equal to 100, got {}.",
        //value);
        //}
        if value < 1 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be greater than or equal to 1, got {}.", value);
        }

        Guess {
            value
        }
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I go the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;


    //#[test]
    //fn exploration() {
        //assert_eq!(2 + 2, 4);
    //}

    //#[test]
    //fn another() {
        //panic!("Make this tset fail");
    //}

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle{ length: 8, width: 7 };
        let smaller = Rectangle{ length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    //#[test]
    //fn it_adds_two() {
        //assert_eq!(4, add_two(2));
    //}

    //#[test]
    //fn greeting_contains_name() {
        //let result = greeting("Carol");
        //assert!(
            //result.contains("Carol"),
            //"Greetintg did not contain name, value was `{}`", result
        //);
    //}

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
