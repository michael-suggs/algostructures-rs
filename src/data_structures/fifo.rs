//! First-in, First-out.

use std::cell::{Ref, RefCell};

trait FIFO<T> {
    fn new() -> Self;
    fn size(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn first(&self) -> Option<Ref<T>>;
    fn enqueue(&mut self, elem: T);
    fn dequeue(&mut self) -> Option<T>;
}
