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
    data: i32,
    next: Option<Weak<InternalNode>>,
    prev: Option<Weak<InternalNode>>,
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
            Some(_) => 1,
        }
    }

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

    fn append_subsequent(&mut self, _data: i32) {
        panic!("not implemented!");
    }
}

impl NodeView {
    fn new(source: &Rc<InternalNode>) -> Self {
        NodeView {
            data: source.data,
            prev: source
                .prev
                .borrow()
                .as_ref()
                .and_then(|ref p| p.upgrade())
                .map(|ref p| Rc::downgrade(p)),
            next: source.next.borrow().as_ref().map(|ref n| Rc::downgrade(n)),
        }
    }

    pub fn value(&self) -> i32 {
        self.data
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
        println!("{:?}", l);
        assert_eq!(1, l.len());
    }

    #[test]
    fn can_get_that_item_from_either_side() {
        let mut l = DoublyLinkedList::new();
        l.append(1);
        println!("{:?}", l);

        let a = l.first().unwrap();
        assert_eq!(1, a.value());

        let b = l.last().unwrap();
        assert_eq!(1, b.value());
    }
}
