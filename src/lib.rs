use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct DoublyLinkedList {
    first: RefCell<Option<Rc<DoublyLinkedListNode>>>,
    last: RefCell<Weak<DoublyLinkedListNode>>,
}

pub struct DoublyLinkedListNode {
    data: i32,
    next: RefCell<Option<Rc<DoublyLinkedListNode>>>,
    pref: RefCell<Weak<DoublyLinkedListNode>>,
}

impl DoublyLinkedList {
    fn new() -> Self {
        DoublyLinkedList {
            first: RefCell::new(None),
            last: RefCell::new(Weak::new()),
        }
    }

    pub fn len(&self) -> usize {
        match *self.first.borrow() {
            None => 0,
            Some(_) => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DoublyLinkedList;

    #[test]
    fn it_works() {
        let l = DoublyLinkedList::new();
        assert_eq!(0, l.len());
    }
}
