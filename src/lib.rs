//! Doubly-linked list in Rust.
//!
//! You were right.
//!
//! This is hard.
//!
//! And completely pointless.
//!
//!
//! # Examples
//!
//! ```
//! use dll::prelude::*;
//!
//! // Create a DoublyLinkedList using dll![]
//! let mut l = dll!['🙈', '🙉', '🙊'];
//! assert_eq!(3, l.len());
//!
//! // Append a value to the end; it becomes the last value.
//! l.append('🚀');
//! assert_eq!(4, l.len());
//! assert_eq!('🚀', l.last().unwrap().value());
//!
//! // Prepend a value to the beginning; it becomes the first value.
//! l.prepend('🛑');
//! assert_eq!(5, l.len());
//! assert_eq!('🛑', l.first().unwrap().value());
//! ```
//!
//! Create a [DoublyLinkedList] from an existing collection:
//!
//! ```
//! use dll::prelude::*;
//!
//! let a = [1, 2, 3];
//! let list: DoublyLinkedList<_> = a.iter().map(|x| x * x).collect();
//! assert_eq!(dll![1, 4, 9], list);
//! ```

use std::cell::RefCell;
use std::cmp::PartialEq;
use std::iter::{FromIterator, Iterator};
use std::rc::{Rc, Weak};

pub mod prelude;

/// Initialize a [DoublyLinkedList] with 0 or more items.
///
/// # Examples
///
/// ```
/// use dll::{dll, DoublyLinkedList};
/// let l = dll![1, 2, 3];
/// assert_eq!(3, l.len());
/// ```
///
/// You will need to annotate the type if you're creating an empty [DoublyLinkedList]:
///
/// ```
/// use dll::{dll, DoublyLinkedList};
/// let l: DoublyLinkedList<u32> = dll![];
/// assert_eq!(0, l.len());
/// ```
#[macro_export]
macro_rules! dll {
    [] => {
        DoublyLinkedList::new()
    };
    [$($ex: expr),+] => {
        {
            let mut l = DoublyLinkedList::new();
            $(
                l.append($ex);
            )+
            l
        }
    };
}

/// A discontiguous, ordered, and growable container.
///
/// Offers:
///  * O(1) worst-case insertion at the front
///  * O(1) worst-case insertion at the rear
///  * O(_n_) worst-case indexing arbitrary elements in the list
///
/// but is otherwise really annoying!
#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    first: RefCell<Option<Rc<InternalNode<T>>>>,
    last: RefCell<Option<Weak<InternalNode<T>>>>,
}

/// Owns its next pointer.
#[derive(Debug)]
struct InternalNode<T> {
    data: T,
    next: RefCell<Option<Rc<InternalNode<T>>>>,
    prev: RefCell<Option<Weak<InternalNode<T>>>>,
}

/// A "view" of a particular positing within the [DoublyLinkedList].
///
/// This allows you to access the value without owning the [DoublyLinkedList] internals.
#[derive(Debug, Clone)]
pub struct NodeView<T> {
    node: Rc<InternalNode<T>>,
}

/// Iterates over a [DoublyLinkedList].
pub struct DoublyLinkedListIterator<'a, T>
where
    T: 'a,
{
    list: &'a DoublyLinkedList<T>,
    node: Option<NodeView<T>>,
}

impl<T> DoublyLinkedList<T>
where
    T: Copy,
{
    /// Create an empty [DoublyLinkedList].
    pub fn new() -> Self {
        DoublyLinkedList {
            first: RefCell::new(None),
            last: RefCell::new(None),
        }
    }

    /// Returns a [NodeView] to the first element in the list or `None` if the list is empty.
    pub fn first(&self) -> Option<NodeView<T>> {
        self.first.borrow().as_ref().map(|ref_| NodeView::new(ref_))
    }

    /// Returns a [NodeView] to the last element in the list or `None` if the list is empty.
    pub fn last(&self) -> Option<NodeView<T>> {
        self.last
            .borrow()
            .as_ref()
            .and_then(|weak| weak.upgrade())
            .map(|ref ref_| NodeView::new(ref_))
    }

    /// Returns how many elements are in the list.
    pub fn len(&self) -> usize {
        match *self.first.borrow() {
            None => 0,
            Some(ref node) => node.len_acc(1),
        }
    }

    /// Returns an [Iterator] over the [DoublyLinkedListIterator].
    ///
    /// # Examples
    ///
    /// ```
    /// use dll::prelude::*;
    /// let list = dll![1, 2, 3];
    ///
    /// let mut last = list.first().unwrap().value();
    /// for i in list.iter().skip(1) {
    ///     assert!(i > last);
    ///     last = i;
    /// }
    /// ```
    pub fn iter<'a>(&'a self) -> DoublyLinkedListIterator<'a, T> {
        DoublyLinkedListIterator::new(self)
    }

    /// Append a value to the end of the list.
    pub fn append(&mut self, data: T) {
        let is_empty = matches!(*self.first.borrow(), None);
        if is_empty {
            self.insert_first(data)
        } else {
            self.append_subsequent(data)
        }
    }

    /// Prepend a value to the beginning of the list.
    pub fn prepend(&mut self, data: T) {
        let is_empty = matches!(*self.first.borrow(), None);
        if is_empty {
            self.insert_first(data)
        } else {
            self.prepend_subsequent(data)
        }
    }

    // Private methods

    fn insert_first(&mut self, data: T) {
        let node = Rc::new(InternalNode {
            data,
            prev: RefCell::new(None),
            next: RefCell::new(None),
        });

        *self.first.borrow_mut() = Some(Rc::clone(&node));
        *self.last.borrow_mut() = Some(Rc::downgrade(&node));
    }

    fn append_subsequent(&mut self, data: T) {
        let last = self
            .last
            .borrow()
            .as_ref()
            .and_then(|ref weak| weak.upgrade())
            .unwrap();

        let node = Rc::new(InternalNode {
            data,
            prev: RefCell::new(Some(Rc::downgrade(&last))),
            next: RefCell::new(None),
        });

        *last.next.borrow_mut() = Some(Rc::clone(&node));
        *self.last.borrow_mut() = Some(Rc::downgrade(&node));
    }

    fn prepend_subsequent(&mut self, data: T) {
        let first = Rc::clone(self.first.borrow().as_ref().unwrap());

        let node = Rc::new(InternalNode {
            data,
            prev: RefCell::new(None),
            next: RefCell::new(Some(Rc::clone(&first))),
        });

        *first.prev.borrow_mut() = Some(Rc::downgrade(&node));
        *self.first.borrow_mut() = Some(Rc::clone(&node));
    }
}

impl<T> PartialEq for DoublyLinkedList<T>
where
    T: Copy + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        // early-out when the lengths differ
        if self.len() != other.len() {
            return false;
        }

        // lengths are the same... gotta check each item
        for (a, b) in self.iter().zip(other.iter()) {
            if a.eq(&b) {
                continue;
            }
            return false;
        }

        return true;
    }
}

impl<T> Eq for DoublyLinkedList<T> where T: Copy + Eq {}

impl<T> InternalNode<T>
where
    T: Copy,
{
    // calculate length via tail-recursion and accumulator
    fn len_acc(&self, acc: usize) -> usize {
        match *self.next.borrow() {
            None => acc,
            Some(ref next) => next.len_acc(acc + 1),
        }
    }
}

impl<T> NodeView<T>
where
    T: Copy,
{
    fn new(source: &Rc<InternalNode<T>>) -> Self {
        NodeView {
            node: Rc::clone(source),
        }
    }

    /// Return the value from this point in the list.
    pub fn value(&self) -> T {
        self.node.data
    }

    /// Return a [NodeView] of the next item in list, or `None` if this is the last item in the list.
    pub fn next(&self) -> Option<NodeView<T>> {
        self.node
            .next
            .borrow()
            .as_ref()
            .map(|ref r| NodeView::new(r))
    }

    /// Return a [NodeView] of the previous item in list, or `None` if this is the first item in the list.
    pub fn prev(&self) -> Option<NodeView<T>> {
        self.node
            .prev
            .borrow()
            .as_ref()
            .and_then(|ref weak| weak.upgrade())
            .map(|ref r| NodeView::new(r))
    }
}

impl<'a, T> DoublyLinkedListIterator<'a, T> {
    fn new(list: &'a DoublyLinkedList<T>) -> Self {
        DoublyLinkedListIterator { list, node: None }
    }
}

impl<'a, T> Iterator for DoublyLinkedListIterator<'a, T>
where
    T: 'a + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.node {
            // no traversal
            None => {
                let first = self.list.first();
                self.node = first.clone();
                first.map(|node| node.value())
            }
            // traversal in progress
            Some(ref node) => {
                let next = node.next();
                self.node = next.clone();
                next.map(|node| node.value())
            }
        }
    }
}

impl<T> FromIterator<T> for DoublyLinkedList<T>
where
    T: Copy,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list: DoublyLinkedList<T> = DoublyLinkedList::new();
        for i in iter {
            list.append(i);
        }

        list
    }
}

#[cfg(test)]
mod tests {
    use crate::DoublyLinkedList;

    #[test]
    fn empty_has_len_0() {
        let l: DoublyLinkedList<i32> = DoublyLinkedList::new();
        assert_eq!(0, l.len());
    }

    #[test]
    fn can_append_an_item() {
        let mut l = DoublyLinkedList::new();
        l.append('a');
        assert_eq!(1, l.len());
    }

    #[test]
    fn can_prepend_an_item() {
        let mut l = DoublyLinkedList::new();
        l.prepend('z');
        assert_eq!(1, l.len());
    }

    #[test]
    fn can_prepend_multiple_items() {
        let mut l = DoublyLinkedList::new();
        l.prepend('z');
        l.prepend('y');

        assert_eq!(2, l.len());
    }

    #[test]
    fn can_get_that_item_from_either_side() {
        let mut l = DoublyLinkedList::new();
        l.append('a');

        let a = l.first().unwrap();
        assert_eq!('a', a.value());

        let b = l.last().unwrap();
        assert_eq!('a', b.value());
    }

    #[test]
    fn can_append_two_items() {
        let mut l = DoublyLinkedList::new();
        l.append('a');
        l.append('b');

        assert_eq!(2, l.len());

        let first = l.first().unwrap();
        let last = l.last().unwrap();
        assert_eq!('a', first.value());
        assert_eq!('b', last.value());
    }

    #[test]
    fn can_traverse_list_forward() {
        let mut l = DoublyLinkedList::new();
        l.append('a');
        l.append('b');

        let first = l.first().unwrap();
        let last = first.next().unwrap();
        assert_eq!('b', last.value());
    }

    #[test]
    fn can_traverse_list_backward() {
        let mut l = DoublyLinkedList::new();
        l.append('a');
        l.append('b');

        let last = l.last().unwrap();
        let first = last.prev().unwrap();
        assert_eq!('a', first.value());
    }

    #[test]
    fn can_use_the_macro_empty() {
        let l: DoublyLinkedList<u128> = dll![];
        assert_eq!(0, l.len());
    }

    #[test]
    fn can_use_macro_with_mulitple_values() {
        let l = dll!['a', 'b', 'c', 'x', 'y', 'z'];
        assert_eq!(6, l.len());
        assert_eq!('a', l.first().unwrap().value());
        assert_eq!('z', l.last().unwrap().value());
    }

    #[test]
    fn can_iterate_over_a_list() {
        let l = dll![2, 3, 4];
        let squares: Vec<_> = l.iter().map(|x| x * x).collect();
        assert_eq!(vec![4, 9, 16], squares);
    }

    #[test]
    fn can_be_created_from_a_slice() {
        let a = ['a', 'b', 'c'];
        let list: DoublyLinkedList<_> = a.iter().collect();
        assert_eq!(3, list.len());
    }

    #[test]
    fn can_be_compared_two_identical_dlls() {
        let xs: DoublyLinkedList<_> = (1..10).into_iter().collect();
        let ys: DoublyLinkedList<_> = (1..10).into_iter().collect();

        assert_eq!(xs, ys);
    }

    #[test]
    fn can_be_compared_two_distinct_lists() {
        let mut xs: DoublyLinkedList<_> = (1..10).into_iter().collect();
        let ys: DoublyLinkedList<_> = (1..10).into_iter().collect();

        xs.append(10);

        assert_ne!(xs, ys);
    }

    #[test]
    fn its_equal_to_itself() {
        let list: DoublyLinkedList<_> = (1..10).into_iter().collect();
        assert_eq!(list, list);
    }
}
