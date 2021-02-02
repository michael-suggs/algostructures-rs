use std::ops::Index;
use std::rc::Rc;
use std::vec::Vec;

trait Heap<T> {
    fn new() -> Self;
    fn build_heap(&mut self) -> Self;
    fn heapify(&mut self, i: usize) -> Self;
    fn heapsort(items: Vec<T>) -> Self;
    fn push(&mut self, item: T);
    fn pop(&mut self) -> T;
    fn size(&self) -> usize;
    fn parent(i: usize) -> usize { i / 2 }
    fn left(i: usize) -> usize { 2 * i }
    fn right(i: usize) -> usize { (2 * i) + 1 }
}

struct HeapNode<T> {
    pub key: usize,
    pub val: T,
    pub parent: Option<Rc<HeapNode<T>>>,
    pub left: Option<Rc<HeapNode<T>>>,
    pub right: Option<Rc<HeapNode<T>>>,
}

struct MinHeap<T> {
    pub size: usize,
    pub items: Vec<T>,
    property: fn(&T, &T) -> bool,
}

impl<T: Default + Ord> Heap<T> for MinHeap<T> {
    fn new() -> Self {
        MinHeap {
            size: 0,
            items: Vec::<T>::new(),
            property: |a: &T, b: &T| -> bool { a < b },
        }
    }

    fn build_heap(&mut self) -> Self {
        unimplemented!()
    }

    fn heapify(&mut self, i: usize) -> Self {
        unimplemented!()
    }

    fn heapsort(items: Vec<T>) -> Self {
        MinHeap {
            size: items.len(),
            items: items,
            property: |a: &T, b: &T| -> bool { a < b },
        }.build_heap()
    }

    fn size(&self) -> usize {
        self.size
    }

    fn push(&mut self, item: T) {
        unimplemented!()
    }

    fn pop(&mut self) -> T {
        let root: T = self.items.remove(0);
        self.build_heap();
        root
    }
}

impl<T: Default + Ord> Index<usize> for MinHeap<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.items[idx]
    }
}
