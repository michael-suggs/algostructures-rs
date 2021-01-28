//! List data structures.

use std::ops::{Index, IndexMut};

type Link<T> = Option<Box<Node<T>>>;

pub trait List<T> {
    fn new() -> Self;
    fn len(&self) -> u32;
    fn is_empty(&self) -> bool;
    fn add_first(&mut self, elem: T);
    fn add_last(&mut self, elem: T);
    fn remove_first(&mut self) -> Option<T>;
}

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

    fn link(&mut self, next: Box<Node<T>>) -> Link<T> {
        let prev_next: Option<Box<Node<T>>> = self.next.take();
        self.next = Some(next);
        prev_next
    }

    fn remove(&mut self) -> Link<T> {
        let next_node = self.next.take();
        self.next = None;
        next_node
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
    tail: Link<T>,
    size: u32,
}

impl<T> LinkedList<T> {
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.data
        })
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.data
        })
    }
}

impl<T> List<T> for LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None, tail: None, size: 0 }
    }

    fn len(&self) -> u32 {
        self.size
    }

    fn is_empty(&self) -> bool {
        match self.size {
            0 => true,
            _ => false,
        }
    }

    fn add_first(&mut self, elem: T) {
        let new_node: Box<Node<T>> = Box::new(Node {
            data: elem,
            next: self.head.take()
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    fn add_last(&mut self, elem: T) {
        let new_node: Box<Node<T>> = Box::new(Node {
            data: elem,
            next: None
        });

        match &self.tail {
            Some(n) => {
                n.link(new_node);
                self.size += 1;
            }
            _ => {
                self.head = Some(new_node);
                self.tail = Some(new_node);
                self.size += 1;
            }
        }
    }

    fn remove_first(&mut self) -> Option<T> {
        self.size -= 1;
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node_box) = current {
            current = node_box.next.take();
        }
    }
}

// impl<T> Index<u32> for LinkedList<T> {
//     type Output = Option<&Box<Node<T>>>;

//     fn index(&self, index: u32) -> &Self::Output {
//         let mut i: u32 = 0;
//         let mut current: &Box<Node<T>> = &self.first;

//         while i < index {
//             current = &current.next();
//             i += 1;
//         }

//         &current
//     }
// }

// impl<T> IndexMut<u32> for LinkedList<T> {
//     type Output = &mut Node<T>;

//     fn index_mut(&self, index: u32) -> &Self::Output {
//         let mut i: u32 = 0;
//         let mut current: &mut Node<T> = &mut self.first;

//         while i < index {
//             current = current.next();
//             i += 1;
//         }

//         &mut current
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn peek() {
        let mut slist = LinkedList::new();

        assert_eq!(slist.peek(), None);
        assert_eq!(slist.peek_mut(), None);

        slist.add_first(1);
        slist.add_first(2);
        slist.add_first(3);

        assert_eq!(slist.peek(), Some(&3));
        assert_eq!(slist.peek_mut(), Some(&mut 3));

        slist.peek_mut().map(|val| {
            *val = 42
        });

        assert_eq!(slist.peek(), Some(&42));
        assert_eq!(slist.peek_mut(), Some(&mut 42));
    }
}
