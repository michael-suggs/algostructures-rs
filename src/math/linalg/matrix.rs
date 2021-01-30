//! Generic n-Dimensional Matrix

use generic_array::GenericArray;
use num_traits::Num;
use std::ops::{Add, Div, Mul, Sub};
use std::ops::{Drop, Index, IndexMut};
use std::vec::Vec;

#[derive(PartialEq, Clone)]
pub struct Matrix<T> {
    pub shape: Vec<usize>,
    pub data: Vec<T>,
    index_offsets: Vec<usize>,
}

impl<T> Matrix<T> {
    pub fn new(shape: &Vec<usize>) -> Self {
        Matrix {
            shape: shape.clone(),
            data: Vec::with_capacity(shape.into_iter().fold(1, |t, n| t * n)),
            index_offsets: {
                let mut index_vec: Vec<usize> = vec![1; shape.capacity()];
                for i in 1..shape.capacity() {
                    index_vec[i - 1] = shape[i..].iter().fold(1, |t, n| t * n)
                }
                index_vec[shape.capacity() - 1] = 1;
                index_vec
            }
        }
    }

    pub fn transpose(&self) -> Self {
        unimplemented!()
    }

    pub fn iter_dim(&self, dim: usize) -> &Vec<T> {
        unimplemented!()
    }
}

fn make_index_vec(v: &Vec<usize>) -> Vec<usize> {
    let mut index_vec: Vec<usize> = vec![1; v.capacity()];
    for i in 1..v.capacity() {
        index_vec[i - 1] = v[i..].iter().fold(1, |t, n| t * n)
    }
    index_vec[v.capacity() - 1] = 1;
    index_vec
}

impl<T> Index<Vec<usize>> for Matrix<T> {
    type Output = T;

    fn index(&self, idx: Vec<usize>) -> &Self::Output {
        let matrix_index: usize = idx.iter().zip(self.index_offsets.iter()).map(|(x, y)| x * y).sum();
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
                index_offsets: self.index_offsets
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
            index_offsets: self.index_offsets
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
                index_offsets: self.index_offsets
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
            index_offsets: self.index_offsets
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
//                 shape: self.shape,
//                 data:
//             }
//         }
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
