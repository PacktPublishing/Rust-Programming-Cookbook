
// Rust allows another macro type: derive. It allows to "auto-implement"
// supported traits. Clone, Debug, Copy are typically handy to derive.
#[derive(Clone, Debug, Copy)]
struct MyCustomStruct {
    a: i32,
    b: u32,
    pub c: f32
}

// A typical Rust struct has an impl block for behavior
impl MyCustomStruct {
    
    // The new function is static function, and by convention a constructor
    pub fn new(a: i32, b: u32, c: f32) -> MyCustomStruct {
        MyCustomStruct {
            a: a, b: b, c: c
        }
    }

    // Instance functions feature a "self" reference as the first parameter
    // This self reference can be mutable or owned, just like other variables
    pub fn sum(&self) -> f32 {
        self.a as f32 + self.b as f32 + self.c
    }
}


#[cfg(test)]
mod tests {
    use std::mem;
    use super::MyCustomStruct;

    #[test]
    fn test_custom_struct() {
        // Rust features zero-overhead structs!
        assert_eq!(mem::size_of::<MyCustomStruct>(), 
            mem::size_of::<i32>() + mem::size_of::<u32>() + mem::size_of::<f32>());

        let m = MyCustomStruct::new(1, 2, 3_f32);
        assert_eq!(m.a, 1);
        assert_eq!(m.b, 2);
        assert_eq!(m.c, 3_f32);

        // Let's call the instance method
        assert_eq!(m.sum(), 6_f32);

        // The derived clone trait adds a method clone() and does a deep copy
        let m2 = m.clone();
        // We use the Debug formatter to format the struct
        assert_eq!(format!("{:?}", m2), "MyCustomStruct { a: 1, b: 2, c: 3.0 }");
        
        // This is an implicit (deep) copy, possible only with the Copy trait
        // Added mutability allows to change struct members
        let mut m3 = m;
        
        // As a copy, this should not affect the other instances 
        m3.a = 100;

        // We'll make sure that the values didn't change anywhere else
        assert_eq!(m2.a, 1);
        assert_eq!(m.a, 1);
        assert_eq!(m3.a, 100);
    }

    #[test]
    fn basic_math_stuff() {
        // Works as expected
        assert_eq!(2 + 2, 4);

        // Rust lets you specify the datatype on literals by appending
        // them to the constant. Splitting them by _ is optional.
        assert_eq!(3.14 + 22.86, 26_f32);

        // Some functions are only available on certain types
        assert_eq!(2_i32.pow(2), 4);
        assert_eq!(4_f32.sqrt(), 2_f32);

        // Rust features unsigned variations of integer types
        let a: u64 = 32;
        let b: u64 = 64;

        // Risky, this could overflow
        assert_eq!(b - a, 32);

        // ... this is why there is an overflowing_sub() function available
        assert_eq!(a.overflowing_sub(b), (18446744073709551584, true));

        // By default, Rust variables are immutable, add the mut qualifier
        // to be able to change the value 
        let mut c = 100;
        c += 1;
        assert_eq!(c, 101);
    }

    #[test]
    #[should_panic]
    fn attempt_overflows() {
        let a = 10_u32;
        let b = 11_u32;

        // This will panic since the result is going to be an unsigned
        // type which cannot handle negative numbers
        // Note: _ means ignore the result
        let _ = a - b; 
    }
}
