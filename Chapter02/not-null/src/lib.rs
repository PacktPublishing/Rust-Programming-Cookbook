#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn option_unwrap() {
        // Options to unwrap Options
        assert_eq!(Some(10).unwrap(), 10);
        assert_eq!(None.unwrap_or(10), 10);
        assert_eq!(None.unwrap_or_else(|| 5 * 2), 10);

        // panic ensues
        // Explicitly type None to an Option<i32>
        // expect is always preferred since it has a message attached
        Option::<i32>::None.unwrap();
        Option::<i32>::None.expect("Better say something when panicking");
    }

    #[test]
    fn option_working_with_values() {
        let mut o = Some(42);

        // take the value out and replace it with None
        let nr = o.take();
        assert!(o.is_none());
        // nr now holds an option containing the value
        assert_eq!(nr, Some(42));

        let mut o = Some(42);
        // sometimes it's better to replace the value right away
        assert_eq!(o.replace(1535), Some(42));
        assert_eq!(o, Some(1535));

        let o = Some(1535);
        // the map() function works only on Some() values (not None)
        assert_eq!(o.map(|v| format!("{:#x}", v)), Some("0x5ff".to_owned()));

        let o = Some(1535);
        // Options can be transformed into a Result easily. "Nope" is the Err()
        //  value for the new Result.
        match o.ok_or("Nope") {
            Ok(nr) => assert_eq!(nr, 1535),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn option_sequentials() {
        // Options have a range of abilities and sometimes even behave like sequences
        let a = Some(42);
        let b = Some(1535);
        // boolean logic with options. Note the returned values
        assert_eq!(a.and(b), Some(1535));
        assert_eq!(a.and(Option::<i32>::None), None);
        assert_eq!(a.or(None), Some(42));
        assert_eq!(a.or(b), Some(42));
        assert_eq!(None.or(a), Some(42));

        // chain together Option instances to skip tedious unwrapping
        let new_a = a.and_then(|v| Some(v + 100)).filter(|&v| v != 42);

        // iterate over Options
        assert_eq!(new_a, Some(142));
        let mut a_iter = new_a.iter();
        assert_eq!(a_iter.next(), Some(&142));
        assert_eq!(a_iter.next(), None);
    }

    #[test]
    fn option_pattern_matching() {
        match Some(100) {
            Some(v) => assert_eq!(v, 100),
            None => assert!(false),
        };

        if let Some(v) = Some(42) {
            assert_eq!(v, 42);
        } else {
            assert!(false);
        }
    }
}
