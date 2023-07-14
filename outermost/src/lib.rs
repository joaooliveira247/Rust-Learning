pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function(); // it's work because the father is lib.rs namespace, and a public function
    outermost::middle_secret_function(); // this don't work because is a private function
    outermost::inside::inner_function(); // this don't work because the module is private 
    outermost::inside::secret_function(); // this don't work because the module and function are private
}

#[cfg(test)]
mod tests {
    use super::*;   

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
