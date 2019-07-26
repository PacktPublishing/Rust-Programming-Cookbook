#![allow(unused_variables, unused_macros)]

macro_rules! strange_patterns {
    (The pattern must match precisely) => { "Text" };
    (42) => { "Numeric" };
    (;<=,<=;) => { "Alpha" };
}

macro_rules! compare {
    ($x:literal => $y:block) => { $x == $y };
}

#[derive(Debug)]
pub struct Response(usize);
pub fn register_handler(method: &str, path: &str, handler: &Fn() -> Response ) {
    // do stuff here
}

// Example of more sophisticated pattern matching
macro_rules! web {
    (GET $path:literal => $b:block) => { register_handler("GET", $path, &|| $b) };
    (POST $path:literal => $b:block) => { register_handler("POST", $path, &|| $b) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web() {
        // this doesn't actually do anything, so as long as it compiles, it's fine
        web!(GET "/" => { Response(200) });
        web!(POST "/" => { Response(403) });
    }
 

    #[test]
    fn test_strange_patterns() {
        assert_eq!(strange_patterns!(The pattern must match precisely), "Text");
        assert_eq!(strange_patterns!(42), "Numeric");
        assert_eq!(strange_patterns!(;<=,<=;), "Alpha");
    }

    #[test]
    fn test_compare() {
        assert!(compare!(1 => { 1 }));
    }
}
