//! Generic n-Dimensional Matrix

use std::ops::{Add, Div, Mul, Sub};
use std::ops::{Index}; // Drop, IndexMut
use std::vec::Vec;
use super::MatrixError;

// trait BaseMatrix<T> {
//     fn transpose(&self) -> Self;
//     fn invert(&self) -> Self;
// }

#[derive(PartialEq, Clone)]
pub struct Matrix<T> {
    pub data: Vec<T>,
    pub shape: Vec<usize>,
    stride: Vec<usize>,
}

impl<T: Default + Clone> Matrix<T> {
    pub fn new(shape: Vec<usize>) -> Self {
        Matrix {
            shape: shape.clone(),
            stride: calculate_stride(&shape),
            data: vec![T::default(); shape.into_iter().fold(1, |t, n| t * n)],
            // data: Vec::with_capacity(shape.into_iter().fold(1, |t, n| t * n)),
        }
    }

    pub fn new_with_data(shape: Vec<usize>, data: Vec<T>) -> Self {
        Matrix {
            shape: shape.clone(),
            stride: calculate_stride(&shape),
            data: data,
        }
    }

    // pub fn swap_axes(&self, axes: Vec<usize>) -> Vec<usize> {
    //     let mut new_stride: Vec<usize> = self.stride.clone();
    //     for i in 0..axes.len() {
    //         let tmp: usize = new_stride[i];
    //         new_stride[i] = new_stride[axes[i]];
    //         new_stride[axes[i]] = tmp;
    //     }
    //     new_stride
    // }

    // pub fn swap_axes_mut(&mut self, axes: Vec<usize>) {
    //     for i in 0..axes.len() {
    //         let tmp: usize = self.stride[i];
    //         self.stride[i] = self.stride[axes[i]];
    //         self.stride[axes[i]] = tmp;
    //     }
    // }

    pub fn transpose(&self) -> Self {
        let new_shape: Vec<usize> = self.shape.clone().into_iter().rev().collect();
        let new_stride: Vec<usize> = self.stride.clone().into_iter().rev().collect();


        Matrix {
            stride: new_stride,
            shape: new_shape,
            data: self.data.clone()
        }
    }

    pub fn iter_dim(&self, dim: usize) -> Result<&Vec<T>, MatrixError> {
        unimplemented!()
    }

    pub fn invert(&self) -> Result<Self, MatrixError> {
        unimplemented!()
    }
}

// impl<T: Default> Matrix<T> {
//     pub fn new(shape: &Vec<usize>) -> Self {
//         Matrix {
//             shape: shape.clone(),
//             data: vec![T::default(); shape.into_iter().fold(1, |t, n| t * n)],
//             index_offsets: {
//                 let mut index_vec: Vec<usize> = vec![1; shape.capacity()];
//                 for i in 1..shape.capacity() {
//                     index_vec[i - 1] = shape[i..].iter().fold(1, |t, n| t * n)
//                 }
//                 index_vec[shape.capacity() - 1] = 1;
//                 index_vec
//             }
//         }
//     }
// }

fn calculate_stride(v: &Vec<usize>) -> Vec<usize> {
    let mut index_vec: Vec<usize> = vec![1; v.len()];
    for i in 1..v.len() {
        index_vec[i - 1] = v[i..].iter().fold(1, |t, n| t * n)
    }
    // index_vec[v.capacity() - 1] = 1;
    index_vec
}

impl<T> Index<Vec<usize>> for Matrix<T> {
    type Output = T;

    fn index(&self, idx: Vec<usize>) -> &Self::Output {
        let matrix_index: usize = idx.iter().zip(self.stride.iter()).map(|(x, y)| x * y).sum();
        &self.data[matrix_index]
    }
}

impl<T> Index<usize> for Matrix<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        let mut rem: usize = idx;
        let mut offsets: Vec<usize> = vec![0; self.stride.len()];

        for i in 0..self.stride.len() {
            offsets[i] = rem % &self.stride[i];
            rem -= rem / &self.stride[i];
        }

        let matrix_index: usize = offsets.iter().zip(self.stride.iter()).map(|(x, y)| x * y).sum();
        &self.data[matrix_index]
    }
}

impl<T: Add<Output = T> + Copy> Add for Matrix<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.shape != other.shape {
            panic!("Matrices must have the same dimensions.")
        } else {
            Matrix {
                shape: self.shape,
                data: self.data.into_iter().zip(other.data.into_iter()).map(|(x, y)| x + y).collect(),
                stride: self.stride
            }
        }
    }
}

impl<T: Add<Output = T> + Copy> Add<T> for Matrix<T> {
    type Output = Self;

    fn add(self, other: T) -> Self {
        Matrix {
            shape: self.shape,
            data: self.data.into_iter().map(|x| x + other).collect(),
            stride: self.stride
        }
    }
}

impl<T: Sub<Output = T> + Copy> Sub for Matrix<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.shape != other.shape {
            panic!("Matrices must have the same dimensions.")
        } else {
            Matrix {
                shape: self.shape,
                data: self.data.into_iter().zip(other.data.into_iter()).map(|(x, y)| x - y).collect(),
                stride: self.stride
            }
        }
    }
}

impl<T: Sub<Output = T> + Copy> Sub<T> for Matrix<T> {
    type Output = Self;

    fn sub(self, other: T) -> Self {
        Matrix {
            shape: self.shape,
            data: self.data.into_iter().map(|x| x - other).collect(),
            stride: self.stride
        }
    }
}

// impl<T: Clone + Mul> Mul for Matrix<T> {
//     type Output = Self;

//     fn mul(self, other: Self) -> Self {
//         if self.shape.last() != other.shape.first() {
//             panic!("Matrices cannot be multiplied.")
//         } else {
//             Matrix {
//                 shape: self.shape.last() * self.shape.first(),
//                 data:
//             }
//         }
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    fn setup_matrices() -> Vec<(Vec<usize>, Matrix<i32>)>{
        let s1d: Vec<usize> = vec![5];
        let s2d: Vec<usize> = vec![3, 7];
        let s3d: Vec<usize> = vec![4, 2, 3];

        let matrices: Vec<(Vec<usize>, Matrix<i32>)> = vec![
            (s1d.clone(), Matrix::new(s1d)),
            (s2d.clone(), Matrix::new(s2d)),
            (s3d.clone(), Matrix::new(s3d))
        ];

        matrices
    }

    #[test]
    fn test_matrix_new_sizing() {
        let matrices: Vec<(Vec<usize>, Matrix<i32>)> = setup_matrices();

        for (svec, mx) in matrices {
            assert_eq!(mx.shape, svec);
            assert_eq!(mx.data.capacity(), svec.iter().fold(1, |t, n| t * n));
        }

        // assert_eq!(m1d.shape, s1d);
        // assert_eq!(m1d.data.capacity(), 5);

        // assert_eq!(m2d.shape, s2d);
        // assert_eq!(m2d.data.capacity(), 21);

        // assert_eq!(m3d.shape, s3d);
        // assert_eq!(m3d.data.capacity(), 24);
    }

    #[test]
    fn test_matrix_indexing() {
        let idx1: Vec<usize> = vec![2, 0, 3];
        let vec1: Vec<usize> = vec![3, 4, 5];
        let vec1idx: Vec<usize> = calculate_stride(&vec1);
        println!("{:?}, {:?}", vec1idx, vec1);
        assert_eq!(vec1idx, vec![20, 5, 1]);
    }

    #[test]
    fn test_matrix_add_matrix() {
        unimplemented!()
    }

    #[test]
    fn test_matrix_transpose() {
        let data: Vec<i32> = (0..30).collect();
        let shape: Vec<usize> = vec![2, 3, 5];
        let m1: Matrix<i32> = Matrix::new_with_data(shape, data);

        assert_eq!(
            m1.data,
            vec![0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 16,
                17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29]
        );
        assert_eq!(m1.stride, vec![15, 5, 1]);

        let m1t: Matrix<i32> = m1.transpose();
        let m1t_data: Vec<i32> = vec![0, 15,  5, 20, 10, 25,  1, 16,  6, 21, 11, 26,  2, 17,  7, 22, 12,
        27,  3, 18,  8, 23, 13, 28,  4, 19,  9, 24, 14, 29];
        assert_eq!(m1t.shape, vec![5, 3, 2]);
        assert_eq!(m1t.stride, vec![1, 5, 15]);

        for i in 0..30 {
            assert_eq!(m1t[i], m1t_data[i]);
        }
    }
}
