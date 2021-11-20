use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct DoublyLinkedList {
    first: RefCell<Option<Rc<DoublyLinkedListNode>>>,
    last: RefCell<Option<Weak<DoublyLinkedListNode>>>,
}

#[derive(Debug)]
pub struct DoublyLinkedListNode {
    data: i32,
    next: RefCell<Option<Rc<DoublyLinkedListNode>>>,
    prev: RefCell<Option<Weak<DoublyLinkedListNode>>>,
}

impl DoublyLinkedList {
    pub fn new() -> Self {
        DoublyLinkedList {
            first: RefCell::new(None),
            last: RefCell::new(None),
        }
    }

    pub fn first(&self) -> Option<Rc<DoublyLinkedListNode>> {
        match *self.first.borrow() {
            Some(ref r) => Some(Rc::clone(r)),
            None => None,
        }
    }

    /// Get the last element
    pub fn last(&self) -> Option<Rc<DoublyLinkedListNode>> {
        self.first.borrow().as_ref().map(|r| Rc::clone(&r))
    }

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
        *self.first.borrow_mut() = Some(Rc::new(DoublyLinkedListNode {
            data,
            prev: RefCell::new(None),
            next: RefCell::new(None),
        }));
    }

    fn append_subsequent(&mut self, _data: i32) {
        panic!("not implemented!");
    }
}

impl DoublyLinkedListNode {
    pub fn value(&self) -> i32 {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use crate::DoublyLinkedList;
    use std::rc::Rc;

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

        let a = l.first();
        let a = match a {
            Some(ref value) => value,
            None => panic!("should not get here!"),
        };
        let a = Rc::clone(a);
        assert_eq!(1, a.value());
    }
}
