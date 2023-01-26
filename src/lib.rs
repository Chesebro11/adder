// HOW TO WRITE TESTS
// Set up any needed data or state
// Run the code you want to test
// Assert the results are what you expect

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// #[test] annotation indicates this is a test function
// cargo test command runs all tests in our project
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
// This is the test module and function generated automatically by cargo new

// Modifying default test:
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn exploration() {
//         assert_eq!(2 + 2, 4);
//     }
//     // Writing a failing test
//     #[test]
//     fn another() {
//         panic!("Make this test fail");
//     }
// }

// Checking Results with the asssert! Macro
// assert! macro is useful when you want to ensure that a condition evaluates to true
// the macro gets an argument that evaluates to a boolen if true test passes
// if false the assert! macro calls panic to cause a fail

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width < other.width && self.height > other.height
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };

//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(larger.can_hold(&smaller));
// }
// // bc the correct result of can_hold is false we need to negate that result or it will pass
//     #[test]
//     fn smaller_cannot_hold_larger() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(!smaller.can_hold(&larger));
//     }
// }

// Testing Equallity with the assert_eq! and assert_ne! Macros

// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_adds_two() {
//         assert_eq!(4, add_two(2));
//     }
// }

// Custom Failure Messages

// pub fn greeting(name: &str) -> String {
//     format!("Hello!")
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn greeting_contains_name() {
//         let result = greeting("Carol");
//         assert!(
//             result.contains("Carol"),
//                 // Error message showing what the result contained if failed
//             "Greeting did not contain name, value was '{}'",
//             result
//         );
//     }
// }

// Checking for panics with should_panic

// pub struct Guess {
//     value: i32,
// }

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 {
//             panic!(
//                 "Guess value must be greater than or equal to 1, got {}.",
//                 value
//             );
//         } else if value > 100 {
//             panic!(
//                 "Guess value must be less than or equal to 100, got {}.",
//                 value
//             );
//         }

//         Guess { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic(expected = "less than or equal to 100")]
//     fn greater_than_100() {
//         Guess::new(200);
//     }
// }