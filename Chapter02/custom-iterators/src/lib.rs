//!
//! A simple singly-linked list for the Rust-Cookbook by Packt Publishing.
//!
//! Recipes covered in this module:
//!  - Documenting your code
//!  - Testing your documentation
//!  - Writing tests and benchmarks
//!

#![feature(test)]
#![doc(
    html_logo_url = "https://blog.x5ff.xyz/img/main/logo.png",
    test(no_crate_inject, attr(allow(unused_variables), deny(warnings)))
)]

use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone)]
struct Node<T>
where
    T: Sized + Clone,
{
    value: T,
    next: Link<T>,
}

impl<T> Node<T>
where
    T: Sized + Clone,
{
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}

///
/// A singly-linked list, with nodes allocated on the heap using `Rc`s and `RefCell`s. Here's an image illustrating a linked list:
///
///
/// ![](https://upload.wikimedia.org/wikipedia/commons/6/6d/Singly-linked-list.svg)
///
/// *Found on https://en.wikipedia.org/wiki/Linked_list*
///
/// # Usage
///
/// ```ignore
/// let list = List::new_empty();
/// ```
///
#[derive(Clone)]
pub struct List<T>
where
    T: Sized + Clone,
{
    head: Link<T>,
    tail: Link<T>,

    ///
    /// The length of the list.
    ///
    pub length: usize,
}

impl<T> List<T>
where
    T: Sized + Clone,
{
    ///
    /// Creates a new empty list.
    ///
    ///  
    /// # Example
    ///
    /// ```
    /// # use custom_iterators::List;
    /// let list: List<i32> = List::new_empty();
    /// ```
    ///
    pub fn new_empty() -> List<T> {
        List {
            head: None,
            tail: None,
            length: 0,
        }
    }

    ///
    /// Appends a node to the list at the end.
    ///
    ///  
    /// # Panics
    ///
    /// This never panics (probably).
    ///
    /// # Safety
    ///
    /// No unsafe code was used.
    ///
    /// # Example
    ///
    /// ```
    /// use custom_iterators::List;
    ///
    /// let mut list = List::new_empty();
    /// list.append(10);
    /// ```
    ///
    pub fn append(&mut self, value: T) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        self.tail = Some(new);
    }

    ///
    /// Removes the list's head and returns the result.
    ///
    ///  
    /// # Panics
    ///
    /// Whenever when a node unexpectedly is `None`
    ///
    /// # Example
    ///
    /// ```
    /// # use custom_iterators::List;
    ///
    /// let mut list = List::new_empty();
    /// list.append(10);
    /// assert_eq!(list.pop_front(), Some(10));
    /// ```
    ///
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value
        })
    }

    ///
    /// Splits off and returns `n` nodes as a `List<T>`.  
    ///
    /// # Arguments
    ///
    /// `n: usize` - The number of elements after which to split the list.
    ///
    /// # Panics
    ///
    /// Panics when:
    ///  - The list is empty
    ///  - `n` is larger than the length
    ///
    /// # Example
    ///
    /// ```
    /// # use custom_iterators::List;
    ///
    /// let mut list = List::new_empty();
    /// list.append(12);
    /// list.append(11);
    /// list.append(10);
    /// let mut list2 = list.split(1);
    /// assert_eq!(list2.pop_front(), Some(12));
    /// assert_eq!(list.pop_front(), Some(11));
    /// assert_eq!(list.pop_front(), Some(10));
    /// ```
    ///
    pub fn split(&mut self, n: usize) -> List<T> {
        // Don't do this in real life. Use Results, Options, or anything that
        // doesn't just kill the program
        if self.length == 0 || n >= self.length - 1 {
            panic!("That's not working");
        }

        let mut n = n;
        let mut new_list = List::new_empty();
        while n > 0 {
            new_list.append(self.pop_front().unwrap());
            n -= 1;
        }
        new_list
    }
}

impl<T> Drop for List<T>
where
    T: Clone + Sized,
{
    fn drop(&mut self) {
        while self.length > 0 {
            let n = self.pop_front();
            drop(n);
        }
    }
}

///
/// An iterator for the linked list. Consumes the list.
///
pub struct ConsumingListIterator<T>
where
    T: Clone + Sized,
{
    list: List<T>,
}

impl<T> ConsumingListIterator<T>
where
    T: Clone + Sized,
{
    ///
    /// Create a new iterator for this list
    ///
    fn new(list: List<T>) -> ConsumingListIterator<T> {
        ConsumingListIterator { list: list }
    }
}

impl<T> Iterator for ConsumingListIterator<T>
where
    T: Clone + Sized,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.list.pop_front()
    }
}

impl<T> IntoIterator for List<T>
where
    T: Clone + Sized,
{
    type Item = T;
    type IntoIter = ConsumingListIterator<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        ConsumingListIterator::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_list_append(b: &mut Bencher) {
        let mut list = List::new_empty();
        b.iter(|| {
            list.append(10);
        });
    }

    fn new_list(n: usize, value: Option<usize>) -> List<usize> {
        let mut list = List::new_empty();
        for i in 1..=n {
            if let Some(v) = value {
                list.append(v);
            } else {
                list.append(i);
            }
        }
        return list;
    }

    #[test]
    fn test_list_iterator() {
        let list = new_list(4, None);
        assert_eq!(list.length, 4);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), None);

        let list = new_list(4, Some(1));
        assert_eq!(list.length, 4);

        for item in list {
            assert_eq!(item, 1);
        }

        let list = new_list(4, Some(1));
        assert_eq!(list.length, 4);
        assert_eq!(list.into_iter().fold(0, |s, e| s + e), 4);
    }

    #[test]
    fn test_list_new_empty() {
        let mut list: List<i32> = List::new_empty();
        assert_eq!(list.length, 0);
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_list_append() {
        let mut list = List::new_empty();
        list.append(1);
        list.append(1);
        list.append(1);
        list.append(1);
        list.append(1);
        assert_eq!(list.length, 5);
    }

    #[test]
    fn test_list_pop_front() {
        let mut list = List::new_empty();
        list.append(1);
        list.append(1);
        list.append(1);
        list.append(1);
        list.append(1);
        assert_eq!(list.length, 5);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.length, 0);
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_list_split() {
        let mut list = List::new_empty();
        list.append(1);
        list.append(1);
        list.append(1);
        list.append(1);
        list.append(1);
        assert_eq!(list.length, 5);
        let list2 = list.split(3);
        assert_eq!(list.length, 2);
        assert_eq!(list2.length, 3);
    }

    #[test]
    #[should_panic]
    fn test_list_split_panics() {
        let mut list: List<i32> = List::new_empty();
        let _ = list.split(3);
    }
}
