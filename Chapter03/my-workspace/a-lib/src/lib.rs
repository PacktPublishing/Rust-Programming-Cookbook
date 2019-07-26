use std::fmt::Debug;

pub fn stringify<T: Debug>(v: &T) -> String {
    format!("{:#?}", v)
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;
    use super::stringify;
    
    #[test]
    fn test_numbers() {        
        let a_nr: f64 = random();
        assert_eq!(stringify(&a_nr), format!("{:#?}", a_nr));
        assert_eq!(stringify(&1i32), "1");
        assert_eq!(stringify(&1usize), "1");
        assert_eq!(stringify(&1u32), "1");
        assert_eq!(stringify(&1i64), "1");
    }

    #[test]
    fn test_sequences() {
        assert_eq!(stringify(&vec![0, 1, 2]), "[\n    0,\n    1,\n    2,\n]");
        assert_eq!(
            stringify(&(1, 2, 3, 4)),
            "(\n    1,\n    2,\n    3,\n    4,\n)"
        );
    }
}
