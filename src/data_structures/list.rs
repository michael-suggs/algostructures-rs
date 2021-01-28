//! List data structures.

use std::ops::{Index, IndexMut};
use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;
type DoubleLink<T> = Option<Rc<RefCell<DoubleNode<T>>>>;

#[derive(Clone)]
pub struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            data: data,
            next: None,
        }
    }

    fn next(&self) -> &Link<T> {
        &self.next
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self as *const _ == other as *const _
    }
}

#[derive(Clone)]
pub struct LinkedList<T> {
    head: Link<T>,
    size: usize,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None, size: 0}
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.data
        })
    }

    fn tail(&self) -> LinkedList<T> {
        LinkedList {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
            size: self.size - 1,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(node) = current {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                current = node.next.take();
            } else {
                break;
            }
        }
    }
}

struct DoubleNode<T> {
    data: T,
    next: DoubleLink<T>,
    prev: DoubleLink<T>,
}

impl<T> DoubleNode<T> {
    fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(DoubleNode {
            data: data,
            next: None,
            prev: None,
        }))
    }
}

struct DoublyLinkedList<T> {
    head: DoubleLink<T>,
    tail: DoubleLink<T>,
    size: usize,
}

impl<T> DoublyLinkedList<T> {
    fn new() -> Self {
        DoublyLinkedList { head: None, tail: None, size: 0 }
    }

    fn add_front(&mut self, data: T) {
        let new_head = DoubleNode::new(data);
        match self.head.take() {
            Some(n) => {
                n.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(n);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }
}

#[cfg(test)]
mod test {
    // use super::*;

    // #[test]
    // fn peek() {
    //     let mut slist = LinkedList::new();

    //     assert_eq!(slist.head(), None);
    //     assert_eq!(slist.head_mut(), None);

    //     slist.add_first(1);
    //     slist.add_first(2);
    //     slist.add_first(3);

    //     assert_eq!(slist.head(), Some(&3));
    //     assert_eq!(slist.head_mut(), Some(&mut 3));

    //     slist.head_mut().map(|val| {
    //         *val = 42
    //     });

    //     assert_eq!(slist.head(), Some(&42));
    //     assert_eq!(slist.head_mut(), Some(&mut 42));
    // }
}
