//! Sorting algorithms.

fn insertion_sort<T: PartialOrd>(array: &mut [T]) {
    for j in 1..array.len() {
        let key: &T = &array[j];
        let mut i: usize = j - 1;

        while (i > 0) && (array[i] > *key) {
            array[i + 1] = array[i];
            i = i - 1;
        }

        array[i + 1] = *key;
    }
}
