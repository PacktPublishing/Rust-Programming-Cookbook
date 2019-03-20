#[cfg(test)]
mod tests {
    #[test]
    fn conditionals() {
        let i = 20;
        // Rust's if statement does not require parenthesis
        if i < 2 {
            assert!(i < 2);
        } else if i > 2 {
            assert!(i > 2);
        } else {
            assert_eq!(i, 2);
        }
    }

    #[test]
    fn loops() {

        let mut i = 42;
        let mut broke = false;
        
        // a basic loop with control statements
        loop {
            i -= 1;
            if i < 2 {
                broke = true;
                break;
            } else if i > 2 {
                continue;
            }
        }
        assert!(broke);

        // loops and other constructs can be named for better readability ...
        'outer: loop  {
            'inner: loop  {
                break 'inner; // ... and specifically jumped out of
            }
            break 'outer;
        }
        let mut iterations: u32 = 0;

        // loops can even have return values on breaks
        let total_squared = loop {
            iterations += 1;

            if iterations >= 10 {
                break iterations.pow(2);
            }
        };
        assert_eq!(total_squared, 100);

        // for loops can use ranges ...
        for i in 0..10 { 
            assert!(i >= 0 && i < 10)
        }

        // or anything that is an iterator
        for v in vec![1, 1, 1, 1].iter() {
            assert_eq!(v, &1);
        }
    }

    #[test]
    fn more_conditionals() {
        let my_option = Some(10);

        // If let statements can do simple pattern matching
        if let Some(unpacked) = my_option {
            assert_eq!(unpacked, 10);
        } 

        let mut other_option = Some(2);
        // there is also while let, which does the same thing
        while let Some(unpacked) = other_option {

            // if can also return values in assignments
            other_option = if unpacked > 0 {
                Some(unpacked - 1)
            } else { 
                None
            }
        }
        assert_eq!(other_option, None)
    }
}
