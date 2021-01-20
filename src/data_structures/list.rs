//! List data structures.

pub trait List<T> {
    fn len(&self) -> u32;
    fn is_empty(&self) -> bool;
    fn add_first(&self, node: Box<Node<T>>);
    fn add_last(&self, node: Box<Node<T>>);
    fn remove_first(&self) -> Option<Node<T>>;
}

#[derive(Clone)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            data: data,
            next: None,
        }
    }

    fn link(&mut self, next: Box<Node<T>>) {
        self.next = Some(next);
    }

    fn next(&self) -> Option<Box<Node<T>>> {
        self.next
    }
}

#[derive(Clone)]
pub struct LinkedList<T> {
    first: Option<Box<Node<T>>>,
    last: Option<Box<Node<T>>>,
    size: u32,
}

impl<T> List<T> for LinkedList<T> {
    fn len(&self) -> u32 {
        self.size
    }

    fn is_empty(&self) -> bool {
        match self.first {
            Some(x) => false,
            _ => true,
        }
    }

    fn add_first(&self, node: Box<Node<T>>) {
        match self.first {
            Some(n) => {
                node.link(n);
                self.first = Some(node);
                self.size += 1;
            },
            _ => {
                self.first = Some(node);
                self.last = Some(node);
                self.size += 1;
            }
        }
    }

    fn add_last(&self, node: Box<Node<T>>) {
        match self.last {
            Some(n) => {
                n.link(node);
                self.size += 1;
            }
            _ => {
                self.first = Some(node);
                self.last = Some(node);
                self.size += 1;
            }
        }
    }

    fn remove_first(&self) -> Option<Node<T>> {
        match self.first {
            Some(n) => {
                assert!(!self.last.is_none());
                let first: Node<T> = *n;
                if n == self.last {
                    self.first = None;
                    self.last = None;
                    return Some(first);
                } else {
                    self.first = n.next();
                }
                self.size -= 1;
                return Some(first);
            }
            _ => None
        }
    }
}

#[cfg(test)]
mod test {
    // use super::*;
    //
    // #[test]
    // fn linked_list_creation() -> LinkedList {

    // }
}
