use rand;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn te   ste_add_one() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
