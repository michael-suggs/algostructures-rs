//! Last-In, First-Out

use std::cell::{Ref, RefCell};

trait LIFO<T> {
    fn new() -> Self;
    fn size(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn top(&self) -> Option<Ref<T>>;
    fn push(&mut self, elem: T);
    fn pop(&mut self) -> Option<T>;
}
