use std::boxed::Box;
use std::cmp;
use std::ops::Index;

///
/// Minimum size of a dynamic array.
///
const MIN_SIZE: usize = 10;

///
/// Type declaration for readability.
///
type Node<T> = Option<T>;

///
/// A dynamic array that can increase in capacity and behaves like
/// a list.
///
pub struct DynamicArray<T>
where
    T: Sized + Clone,
{
    buf: Box<[Node<T>]>,
    cap: usize,
    pub length: usize,
}

impl<T> DynamicArray<T>
where
    T: Sized + Clone,
{
    ///
    /// Create a new empty dynamic array.
    ///
    pub fn new_empty() -> DynamicArray<T> {
        DynamicArray {
            buf: vec![None; MIN_SIZE].into_boxed_slice(),
            length: 0,
            cap: MIN_SIZE,
        }
    }

    ///
    /// A simple growth strategy, that at least doubles the size
    /// whenever expansion is needed. Clones old content into the
    /// new memory.
    ///
    fn grow(&mut self, min_cap: usize) {
        let old_cap = self.buf.len();
        let mut new_cap = old_cap + (old_cap >> 1);

        new_cap = cmp::max(new_cap, min_cap);
        new_cap = cmp::min(new_cap, usize::max_value());
        let current = self.buf.clone();
        self.cap = new_cap;

        self.buf = vec![None; new_cap].into_boxed_slice();
        self.buf[..current.len()].clone_from_slice(&current);
    }

    ///
    /// Append a value to the dynamic array.
    ///
    pub fn append(&mut self, value: T) {
        if self.length == self.cap {
            self.grow(self.length + 1);
        }
        self.buf[self.length] = Some(value);
        self.length += 1;
    }

    ///
    /// Retrieve an element from a specific position.
    /// Clones the element.
    ///
    pub fn at(&mut self, index: usize) -> Node<T> {
        if self.length > index {
            self.buf[index].clone()
        } else {
            None
        }
    }
}

impl<T> Index<usize> for DynamicArray<T>
where
    T: Sized + Clone,
{
    type Output = Node<T>;

    fn index(&self, index: usize) -> &Self::Output {
        if self.length > index {
            &self.buf[index]
        } else {
            &None
        }
    }
}

///
/// Deep copy of the dynamic array.
///
impl<T> Clone for DynamicArray<T>
where
    T: Sized + Clone,
{
    fn clone(&self) -> Self {
        DynamicArray {
            buf: self.buf.clone(),
            cap: self.cap,
            length: self.length,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn dynamic_array_clone() {
        let mut list = DynamicArray::new_empty();
        list.append(3.14);
        let mut list2 = list.clone();
        list2.append(42.0);
        assert_eq!(list[0], Some(3.14));
        assert_eq!(list[1], None);
        
        assert_eq!(list2[0], Some(3.14));
        assert_eq!(list2[1], Some(42.0));
    }


    #[test]
    fn dynamic_array_index() {
        let mut list = DynamicArray::new_empty();
        list.append(3.14);

        assert_eq!(list[0], Some(3.14));
        let mut list = DynamicArray::new_empty();
        list.append("Hello");
        assert_eq!(list[0], Some("Hello"));
        assert_eq!(list[1], None);
    }

    #[test]
    fn dynamic_array_2d_array() {
        let mut list = DynamicArray::new_empty();
        let mut sublist = DynamicArray::new_empty();
        sublist.append(3.14);
        list.append(sublist);

        assert_eq!(list.at(0).unwrap().at(0), Some(3.14));
        assert_eq!(list[0].as_ref().unwrap()[0], Some(3.14));

    }


    #[test]
    fn dynamic_array_append() {
        let mut list = DynamicArray::new_empty();
        let max: usize = 1_000;
        for i in 0..max {
            list.append(i as u64);
        }
        assert_eq!(list.length, max);
    }

    #[test]
    fn dynamic_array_at() {
        let mut list = DynamicArray::new_empty();
        let max: usize = 1_000;
        for i in 0..max {
            list.append(i as u64);
        }
        assert_eq!(list.length, max);
        for i in 0..max {
            assert_eq!(list.at(i), Some(i as u64));
        }
        assert_eq!(list.at(max + 1), None);
    }
}
