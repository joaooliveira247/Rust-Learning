//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
/// # Adds one to the number given.
/// 
/// ## Examples
/// 
/// ```
/// let arg = 5;
/// let answer = docs::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add_one(n: i32) -> i32 {
    n + 1
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add_one(2);
//         assert_eq!(result, 3);
//     }
// }
