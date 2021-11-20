use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct DoublyLinkedList {
    first: RefCell<Option<Rc<InternalNode>>>,
    last: RefCell<Option<Weak<InternalNode>>>,
}

/// Owns its next pointer.
#[derive(Debug)]
pub struct InternalNode {
    data: i32,
    next: RefCell<Option<Rc<InternalNode>>>,
    prev: RefCell<Option<Weak<InternalNode>>>,
}

/// Extracts data from the list.
///
/// Does not own anything in the list.
#[derive(Debug)]
pub struct NodeView {
    node: Rc<InternalNode>,
}

impl DoublyLinkedList {
    pub fn new() -> Self {
        DoublyLinkedList {
            first: RefCell::new(None),
            last: RefCell::new(None),
        }
    }

    /// Maybe get the first element in the node.
    pub fn first(&self) -> Option<NodeView> {
        self.first.borrow().as_ref().map(|ref_| NodeView::new(ref_))
    }

    /// Get the last element
    pub fn last(&self) -> Option<NodeView> {
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
    pub fn append(&mut self, data: i32) {
        let is_empty = matches!(*self.first.borrow(), None);
        if is_empty {
            self.append_first(data)
        } else {
            self.append_subsequent(data)
        }
    }

    fn append_first(&mut self, data: i32) {
        let node = Rc::new(InternalNode {
            data,
            prev: RefCell::new(None),
            next: RefCell::new(None),
        });

        *self.first.borrow_mut() = Some(Rc::clone(&node));
        *self.last.borrow_mut() = Some(Rc::downgrade(&node));
    }

    fn append_subsequent(&mut self, data: i32) {
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
}

impl InternalNode {
    fn len_acc(&self, acc: usize) -> usize {
        match *self.next.borrow() {
            Some(ref next) => next.len_acc(acc + 1),
            None => acc,
        }
    }
}

impl NodeView {
    fn new(source: &Rc<InternalNode>) -> Self {
        NodeView {
            node: Rc::clone(source),
        }
    }

    pub fn value(&self) -> i32 {
        self.node.data
    }

    pub fn next(&self) -> Option<NodeView> {
        self.node
            .next
            .borrow()
            .as_ref()
            .map(|ref r| NodeView::new(r))
    }

    pub fn prev(&self) -> Option<NodeView> {
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
        let l = DoublyLinkedList::new();
        assert_eq!(0, l.len());
    }

    #[test]
    fn can_append_an_item() {
        let mut l = DoublyLinkedList::new();
        l.append(1);
        assert_eq!(1, l.len());
    }

    #[test]
    fn can_get_that_item_from_either_side() {
        let mut l = DoublyLinkedList::new();
        l.append(1);

        let a = l.first().unwrap();
        assert_eq!(1, a.value());

        let b = l.last().unwrap();
        assert_eq!(1, b.value());
    }

    #[test]
    fn can_append_two_items() {
        let mut l = DoublyLinkedList::new();
        l.append(1);
        l.append(2);

        assert_eq!(2, l.len());

        let first = l.first().unwrap();
        let last = l.last().unwrap();
        assert_eq!(1, first.value());
        assert_eq!(2, last.value());
    }

    #[test]
    fn can_traverse_list_forward() {
        let mut l = DoublyLinkedList::new();
        l.append(1);
        l.append(2);

        let first = l.first().unwrap();
        let last = first.next().unwrap();
        assert_eq!(2, last.value());
    }

    #[test]
    fn can_traverse_list_backward() {
        let mut l = DoublyLinkedList::new();
        l.append(1);
        l.append(2);

        let last = l.last().unwrap();
        let first = last.prev().unwrap();
        assert_eq!(1, first.value());
    }
}
