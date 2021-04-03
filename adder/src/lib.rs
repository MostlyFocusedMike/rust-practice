#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(num: i32) -> i32{
    num + 2
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
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

#[cfg(test)] // not sure what this attribute does yet
mod tests {
    use super::*;
    // the #[test] attribute catches and runs things when you run `cargo test`
    // tests fail when panicking, unless you use the should_panic macro
    #[test]
    fn it_works() { // the function name is the name of the test. Not great that we can't use strings
        println!("hello there");
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic] // generic panic
    fn not_gonna_fail() {
        panic!("Make this test fail");
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")] // check expected error message
    fn greater_than_100() {
        Guess::new(200);
    }

    // when you run test it tells you what passes, how many ignored, and on nightly builds "measured" for performance
    // there's also `Doc-tests <name of crate>`, is for the results of any documentation tests. learn more in ch 14

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

    // Assertion macros
    // assert_eq(any, any) - two values, must be equal.
    // asert_ne(any, any) - two values, must not be equal.
        // those two macros will only print the values if test fails, and they say it as left and right (left, right)
        // When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the PartialEq and Debug traits.
    // assert!(bool) - takes one value, must evaluate to true of fails

    // #[test]
    // fn greeting_contains_name() {
    //     let result = greeting("Carol");
    //     assert!(
    //         result.contains("Carol"),
    //         "Greeting did not contain name, value was '{}', wanted '{}'",
    //         result,
    //         "Carol"
    //     );
    // }
    // that prints a much better error to the screen, it takes a string literal, and then whatever arguments you want to pass in


    // Tests don't have to panic, they can also use Result<(), String>, which lets you use the ? inside them
    // can't use should_panic anymore, you should return an Err with a string explaining what's up

    #[test]
    fn it_works_better() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // #[test]
    // fn failed_test() -> Result<(), String> {
    //     if 2 + 2 == 5 {
    //         Ok(())
    //     } else {
    //         Err(String::from("two plus two does not equal five"))
    //     }
    // }
}

