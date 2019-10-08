//! This is a non-optimized implementation of the [bubble sort](https://en.wikipedia.org/wiki/Bubble_sort) algorithm for the book Rust Cookbook by Packt. This implementation also clones the input vector. 
//! 
//! # Examples
//!```
//!# use bubble_sort::bubble_sort;
//! let v = vec![2, 2, 10, 1, 5, 4, 3]; 
//! assert_eq!(bubble_sort(&v), vec![1, 2, 2, 3, 4, 5, 10]);
//!```

///
/// See module level documentation. 
/// 
pub fn bubble_sort<T: PartialOrd + Clone>(collection: &[T]) -> Vec<T> {
    let mut result: Vec<T> = collection.into();
    for _ in 0..result.len() {
        let mut swaps = 0;
        for i in 1..result.len() {
            if result[i - 1] > result[i] {
                result.swap(i - 1, i);
                swaps += 1;
            }
        }
        if swaps == 0 {
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;
     #[test]
    fn test_bubble_sort() {
        assert_eq!(bubble_sort(&vec![9, 8, 7, 6]), vec![6, 7, 8, 9]);
        assert_eq!(bubble_sort(&vec![9_f32, 8_f32, 7_f32, 6_f32]), vec![6_f32, 7_f32, 8_f32, 9_f32]);

        assert_eq!(bubble_sort(&vec!['c','f','a','x']), vec!['a', 'c', 'f', 'x']);

        assert_eq!(bubble_sort(&vec![6, 8, 7, 9]), vec![6, 7, 8, 9]);
        assert_eq!(bubble_sort(&vec![2, 1, 1, 1, 1]), vec![1, 1, 1, 1, 2]);
    }

}
