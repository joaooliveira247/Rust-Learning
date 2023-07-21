// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn exploration() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }

//     #[test]
//     fn another() {
//         panic!("Make this test fail")
//     }
// }
#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width: width, heigth: height }
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width < other.width && self.heigth > other.heigth
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle::new(8,7);
//         let smaller = Rectangle::new(5, 1);
        
//         assert!(larger.can_hold(&smaller));
//     }

//     #[test]
//     fn smaller_cannot_hold_larger() {
//         let larger = Rectangle::new(8, 7);
//         let smaller = Rectangle::new(5, 1);

//         assert!(!smaller.can_hold(&larger))
//     }
// }

#[allow(dead_code)]
pub fn add_two(a: i32) -> i32 {
    a + 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}