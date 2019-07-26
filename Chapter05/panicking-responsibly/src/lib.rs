#[cfg(test)]
mod tests {

    #[test]
    #[should_panic]
    fn test_regular_panic() {
        panic!();
    }

    #[test]
    #[should_panic(expected = "Everything is lost!")]
    fn test_panic_message() {
        panic!("Everything is lost!");
    }

    #[test]
    #[should_panic(expected = "String formatting also works")]
    fn test_panic_format() {
        panic!("{} formatting also works.", "String");
    }

    #[test]
    #[should_panic]
    fn test_panic_return_value() {
        panic!(42);
    }

    #[test]
    #[should_panic]
    fn test_assert() {
        // assert a condition and panic if it's not met
        assert!(1 == 2);
    }

    #[test]
    #[should_panic]
    fn test_assert_eq() {
        // assert a condition and panic if it's not met
        assert_eq!(1, 2);
    }

    #[test]
    #[should_panic]
    fn test_assert_neq() {
        // assert a condition and panic if it's not met
        assert_ne!(1, 1);
    }

    #[test]
    #[should_panic]
    fn test_unwrap() {
        // panics if "None"
        None::<i32>.unwrap();
    }

    #[test]
    #[should_panic(expected = "Unwrap with a message")]
    fn test_expect() {
        None::<i32>.expect("Unwrap with a message");
    }
}
