#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {name}!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        } 
        Guess { value }
    }
}

// Tells rustc to only compile this module when using 'cargo test'.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic]
    fn another() {
        panic!("This test will fail");
    }
    
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
    
        assert!(larger.can_hold(&smaller));
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
        
        assert!(!smaller.can_hold(&larger));
    }
    
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_adds_two_correctly() {
        assert_ne!(5, add_two(2));
    }

    // Custom fail message.
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"), 
            "Greeting did not contain name, value was '{}'",
            result
        );
    }

    #[test]
    //Tests for panics
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    //Tests for panics of specific type
    #[should_panic(expected = "Guess value must be between 1 and 100, got 200.")]
    fn greater_than_100_2() {
        Guess::new(200);
    }

    // Testing Result<T, E>
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    
    #[test]
    // Will only run if 'cargo test' is given the '-- --ignored' flag.
    #[ignore]
    fn ignored() {
        std::thread::sleep(std::time::Duration::from_secs(10));
        assert!(true);
    }
        
}
