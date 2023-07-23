pub fn print_and_return_10(a: i32) -> usize {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn this_test_will_pass() {
        let result = print_and_return_10(10);
        assert_eq!(result, 10);
    }

    #[test]
    fn this_test_will_not_pass() {
        let result = print_and_return_10(5);
        assert_eq!(result, 5)
    }
}
