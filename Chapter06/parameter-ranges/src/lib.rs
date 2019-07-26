#![allow(unused_macros)]


macro_rules! set {
 ( $( $item:expr ),* ) => {
        {
            let mut s = HashSet::new();
            $(
                s.insert($item);
            )*
            s
        }
    };
}

macro_rules! dto {
    ($name: ident, $($variant: ident => $type:ty),+) => {
        #[derive(PartialEq, Clone, Debug)]
        pub struct $name {
            $(pub $variant: $type),+
        }

        impl $name {
            pub fn new($($variant:$type),+) -> Self {
                $name {
                    $($variant: $variant),+
                }
             }
        }
    };
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;


    #[test]
    fn test_set() {
        let actual = set!("a", "b", "c", "a");
        let mut desired = HashSet::new();
        desired.insert("a");
        desired.insert("b");
        desired.insert("c");
        assert_eq!(actual, desired);    
    }

    #[test]
    fn test_dto() {
        dto!(Sensordata, value => f32, timestamp => u64);
        let s = Sensordata::new(1.23f32, 123456);
        assert_eq!(s.value, 1.23f32);    
        assert_eq!(s.timestamp, 123456);    
    }
}
