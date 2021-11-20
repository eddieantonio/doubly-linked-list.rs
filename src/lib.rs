use std::cell::RefCell;
use std::rc::{Rc, Weak};

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

#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    first: RefCell<Option<Rc<InternalNode<T>>>>,
    last: RefCell<Option<Weak<InternalNode<T>>>>,
}

/// Owns its next pointer.
#[derive(Debug)]
pub struct InternalNode<T> {
    data: T,
    next: RefCell<Option<Rc<InternalNode<T>>>>,
    prev: RefCell<Option<Weak<InternalNode<T>>>>,
}

/// Extracts data from the list.
///
/// Does not own anything in the list.
#[derive(Debug)]
pub struct NodeView<T> {
    node: Rc<InternalNode<T>>,
}

impl<T> DoublyLinkedList<T>
where
    T: Copy,
{
    pub fn new() -> Self {
        DoublyLinkedList {
            first: RefCell::new(None),
            last: RefCell::new(None),
        }
    }

    /// Maybe get the first element in the node.
    pub fn first(&self) -> Option<NodeView<T>> {
        self.first.borrow().as_ref().map(|ref_| NodeView::new(ref_))
    }

    /// Get the last element
    pub fn last(&self) -> Option<NodeView<T>> {
        self.last
            .borrow()
            .as_ref()
            .and_then(|weak| weak.upgrade())
            .map(|ref ref_| NodeView::new(ref_))
    }

    /// How many elements are in list?
    pub fn len(&self) -> usize {
        match *self.first.borrow() {
            None => 0,
            Some(ref node) => node.len_acc(1),
        }
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

impl<T> InternalNode<T>
where
    T: Copy,
{
    fn len_acc(&self, acc: usize) -> usize {
        match *self.next.borrow() {
            Some(ref next) => next.len_acc(acc + 1),
            None => acc,
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

    pub fn value(&self) -> T {
        self.node.data
    }

    pub fn next(&self) -> Option<NodeView<T>> {
        self.node
            .next
            .borrow()
            .as_ref()
            .map(|ref r| NodeView::new(r))
    }

    pub fn prev(&self) -> Option<NodeView<T>> {
        self.node
            .prev
            .borrow()
            .as_ref()
            .and_then(|ref weak| weak.upgrade())
            .map(|ref r| NodeView::new(r))
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
}
