



#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.height > other.height 
    }
}

/* #[cfg(test)]
mod tests {
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

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width:5,
            height:1,
        };
        assert!(!smaller.can_hold(&larger));
    }
}

 */
pub fn add_two(a: u64) -> u64 {
    a + 3
}

/* #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}
 */
pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

/* #[cfg(test)]
mod tests {
    use super::*;

        #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }
} */

pub struct Guess {
    value: i32,
}

/* impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        Guess{value}
    }
}
 */

 
/* impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
} 

 */

 impl Guess {
    pub fn new(value:i32) -> Guess {
        if value > 1 {
            panic!("nope");
        } else if value < 100 {
            panic!("nohe");
        }
        Guess{value}
    }
 }

 #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less or more")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

/* #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
} */

