//! Generic n-Dimensional Matrix

use generic_array::GenericArray;
use std::ops::{Index, IndexMut};
use std::vec::Vec;

pub struct Matrix<T: Clone> {
    pub shape: Vec<usize>,
    pub data: Vec<T>,
    index: Vec<usize>,
}

impl<T: Clone> Matrix<T> {
    pub fn new(shape: &Vec<usize>) -> Self {
        Matrix {
            shape: shape.clone(),
            data: Vec::with_capacity(shape.into_iter().fold(1, |t, n| t * n)),
            index: {
                let mut index_vec: Vec<usize> = vec![1; shape.capacity()];
                for i in 1..shape.capacity() {
                    index_vec[i - 1] = shape[i..].into_iter().fold(1, |t, n| t * n)
                }
                index_vec[shape.capacity() - 1] = 1;
                index_vec
            },
        }
    }
}

// fn make_index_vec(v: &Vec<usize>) -> Vec<usize> {
//     let mut index_vec: Vec<usize> = vec![1; v.capacity()];
//     for i in 1..v.capacity() {
//         index_vec[i - 1] = v[i..].into_iter().fold(1, |t, n| t * n)
//     }
//     index_vec[v.capacity() - 1] = 1;
//     index_vec
// }

// impl<T: Clone> Index<Vec<usize>> for Matrix<T> {
//     type Output = Vec<T>;

//     fn index(&self, idx: Vec<usize>) -> &Self::Output {
//         // matrix[i][j][k] = m[i*(N*M) + j*M + k]
//         let mut flat_idx: usize = 0;

//         for i in 0..idx.capacity() {
//             for j in i..std::cmp::min(idx.capacity(), self.data.capacity()) {
//                 flat_idx +=
//             }
//         }
//         &vec![]
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matrix_new_sizing() {
        let s1d: Vec<usize> = vec![5];
        let s2d: Vec<usize> = vec![3, 7];
        let s3d: Vec<usize> = vec![4, 2, 3];

        let m1d: Matrix<i32> = Matrix::new(&s1d);
        let m2d: Matrix<i32> = Matrix::new(&s2d);
        let m3d: Matrix<i32> = Matrix::new(&s3d);

        assert_eq!(m1d.shape, s1d);
        assert_eq!(m1d.data.capacity(), 5);

        assert_eq!(m2d.shape, s2d);
        assert_eq!(m2d.data.capacity(), 21);

        assert_eq!(m3d.shape, s3d);
        assert_eq!(m3d.data.capacity(), 24);
    }

    #[test]
    fn test_matrix_indexing() {
        let idx1: Vec<usize> = vec![2, 0, 3];
        let vec1: Vec<usize> = vec![3, 4, 5];
        let vec1idx: Vec<usize> = make_index_vec(&vec1);
        println!("{:?}, {:?}", vec1idx, vec1);
        assert_eq!(vec1idx, vec![20, 5, 1]);
    }
}
