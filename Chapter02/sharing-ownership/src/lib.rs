#![feature(test)]


#[cfg(test)]
mod tests {
    extern crate test;
    use std::rc::Rc;
    use test::{black_box, Bencher};

    /// 
    /// A length function that takes ownership of the input variable
    /// 
    fn length(s: String) -> usize {
        s.len()
    }

    ///
    /// The same length function, taking ownership of a Rc
    /// 
    fn rc_length(s: Rc<String>) -> usize {
        s.len() // calls to the wrapped object require no additions 
    }

    #[bench]
    fn bench_string_clone(b: &mut Bencher) {
        let s: String = (0..100_000).map(|_| 'a').collect();
        b.iter(|| {
            black_box(length(s.clone()));
        });
    }

    #[bench]
    fn bench_string_rc(b: &mut Bencher) {
        let s: String = (0..100_000).map(|_| 'a').collect();
        let rc_s = Rc::new(s);
        b.iter(|| {
            black_box(rc_length(rc_s.clone()));
        });
    }
    

    #[test]
    fn cloning() {
        let s = "abcdef".to_owned();
        assert_eq!(length(s), 6);
        // s is now "gone", we can't use it anymore
        // therefore we can't use it in a loop either!
        // ... unless we clone s - at a cost! (see benchmark)
        let s = "abcdef".to_owned();

        for _ in 0..10 {
            // clone is typically an expensive deep copy
            assert_eq!(length(s.clone()), 6);
        }
    }

     #[test]
    fn refcounting() {
        let s = Rc::new("abcdef".to_owned());
        // we can clone Rc (reference counters) with low cost
        assert_eq!(rc_length(s.clone()), 6);

        for _ in 0..10 {
            // clone is typically an expensive deep copy
            assert_eq!(rc_length(s.clone()), 6);
        }
    }
}
