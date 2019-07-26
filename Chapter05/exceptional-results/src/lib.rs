
/// 
/// Finds a needle in a haystack, returns -1 on error 
/// 
pub fn bad_practice_find(needle: &str, haystack: &str) -> i32 {
    haystack.find(needle).map(|p| p as i32).unwrap_or(-1)
}

/// 
/// Finds a needle in a haystack, returns None on error 
/// 
pub fn better_find(needle: &str, haystack: &str) -> Option<usize> {
    haystack.find(needle)
}

#[derive(Debug, PartialEq)]
pub enum FindError {
    EmptyNeedle,
    EmptyHaystack,
    NotFound,
}

/// 
/// Finds a needle in a haystack, returns a proper Result 
/// 
pub fn best_find(needle: &str, haystack: &str) -> Result<usize, FindError> {
    if needle.len() <= 0 {
        Err(FindError::EmptyNeedle)
    } else if haystack.len() <= 0 {
        Err(FindError::EmptyHaystack)
    } else {
        haystack.find(needle).map_or(Err(FindError::NotFound), |n| Ok(n))
    }
}

#[cfg(test)]
mod tests {    

    use super::*;

    #[test]
    fn test_bad_practice() {
        assert_eq!(bad_practice_find("a", "hello world"), -1);
        assert_eq!(bad_practice_find("e", "hello world"), 1);
        assert_eq!(bad_practice_find("", "hello world"), 0);
        assert_eq!(bad_practice_find("a", ""), -1);
    }

    #[test]
    fn test_better_practice() {
        assert_eq!(better_find("a", "hello world"), None);
        assert_eq!(better_find("e", "hello world"), Some(1));
        assert_eq!(better_find("", "hello world"), Some(0));   
        assert_eq!(better_find("a", ""), None);  
    }

    #[test]
    fn test_best_practice() {
        assert_eq!(best_find("a", "hello world"), Err(FindError::NotFound));
        assert_eq!(best_find("e", "hello world"), Ok(1));
        assert_eq!(best_find("", "hello world"), Err(FindError::EmptyNeedle));
        assert_eq!(best_find("e", ""), Err(FindError::EmptyHaystack));  
    }
}
