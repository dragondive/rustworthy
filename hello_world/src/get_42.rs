pub fn get_42() -> i32 {
    return 42;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_42() {
        assert_eq!(get_42(), 42);
    }
}