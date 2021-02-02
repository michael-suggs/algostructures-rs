use std::ops::Index;
use std::rc::Rc;
use std::vec::Vec;

trait Heap<T> {
    fn new() -> Self;
    fn heapify(&self) -> Self;
    fn heapify_mut(&mut self);
    fn push(&self, item: T);
    fn pop(&self) -> T;
    fn size(&self) -> usize;
    fn parent(i: usize) -> usize { i / 2 }
    fn left(i: usize) -> usize { 2 * i }
    fn right(i: usize) -> usize { (2 * i) + 1 }
}

struct MinHeap<T> {
    items: Vec<T>,
    size: usize,
    property: fn(&T, &T) -> bool,
}

impl<T: Default> Heap<T> for MinHeap<T> {
    fn new() -> Self {
        MinHeap {
            size: 0,
            items: Vec::<T>::new(),
            property: |a: &T, b: &T| -> bool { a < b }
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn heapify(&self) -> Self {
        unimplemented!()
    }

    fn heapify_mut(&mut self) {
        unimplemented!()
    }

    fn push(&self, item: T) {
        unimplemented!()
    }

    fn pop(&self) -> T {
        unimplemented!()
    }
}

impl<T> MinHeap<T> {

}
