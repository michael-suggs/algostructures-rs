//! List data structures.

use std::cell::{Ref, RefCell, RefMut};
use std::ops::{Index, IndexMut};
use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;
type DoubleLink<T> = Option<Rc<RefCell<DoubleNode<T>>>>;

/// Node for a singly-linked list.
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

/// Singly-Linked List
impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    fn tail(&self) -> LinkedList<T> {
        LinkedList {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
            size: self.size - 1,
        }
    }

    fn add_front(&self, data: T) -> LinkedList<T> {
        LinkedList {
            head: Some(Rc::new(Node {
                data: data,
                next: self.head.clone(),
            })),
            size: self.size + 1,
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
        DoublyLinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }

    fn len(&self) -> usize {
        self.size
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

    fn add_tail(&mut self, data: T) {
        let new_tail = DoubleNode::new(data);
        match self.tail.take() {
            Some(n) => {
                n.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(n);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    fn remove_head(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            match head.borrow_mut().next.take() {
                Some(next) => {
                    next.borrow_mut().prev.take();
                    self.head = Some(next);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(head).ok().unwrap().into_inner().data
        })
    }

    fn remove_tail(&mut self) -> Option<T> {
        self.tail.take().map(|tail| {
            match tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(tail).ok().unwrap().into_inner().data
        })
    }

    fn peek_head(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.data))
    }

    fn peek_head_mut(&mut self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.data))
    }

    fn peek_tail(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.data))
    }

    fn peek_tail_mut(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.data))
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while self.remove_head().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_remove_single() {
        let list: LinkedList<usize> = LinkedList::new();
        assert!(list.head().is_none());

        let list = list.add_front(1).add_front(2).add_front(3);
        assert_eq!(list.head(), Some(&3));
        let list = list.tail();
        assert_eq!(list.head(), Some(&2));
        let list = list.tail();
        assert_eq!(list.head(), Some(&1));
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn peek_double() {
        let mut dlist: DoublyLinkedList<usize> = DoublyLinkedList::new();
        assert!(dlist.peek_head().is_none());
        assert!(dlist.peek_tail().is_none());
        assert!(dlist.peek_head_mut().is_none());
        assert!(dlist.peek_tail_mut().is_none());

        dlist.add_front(1);
        dlist.add_front(2);
        dlist.add_front(3);

        assert_eq!(&*dlist.peek_head().unwrap(), &3);
        assert_eq!(&*dlist.peek_tail().unwrap(), &1);
        assert_eq!(&*dlist.peek_head_mut().unwrap(), &mut 3);
        assert_eq!(&*dlist.peek_tail_mut().unwrap(), &mut 1);
    }

    #[test]
    fn add_remove_double() {
        let mut dlist: DoublyLinkedList<usize> = DoublyLinkedList::new();
        assert_eq!(dlist.remove_head(), None);

        dlist.add_front(1);
        dlist.add_front(2);
        dlist.add_front(3);

        assert_eq!(dlist.remove_head(), Some(3));
        assert_eq!(dlist.remove_head(), Some(2));

        dlist.add_tail(4);
        dlist.add_tail(5);

        assert_eq!(dlist.remove_head(), Some(1));
        assert_eq!(dlist.remove_tail(), Some(5));
        assert_eq!(dlist.remove_tail(), Some(4));
        assert_eq!(dlist.remove_head(), None);
        assert_eq!(dlist.remove_tail(), None);
    }
}
