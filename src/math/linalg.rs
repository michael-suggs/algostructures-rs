//! Linear Algebra

pub mod matrix;

use snafu::Snafu;
use std::vec::Vec;

type MatrixResult<T, E = MatrixError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum MatrixError {
    #[snafu(display("Matrix dimensions {:?} do not agree with {:?} for operation {:?}", shape_a, shape_b, op))]
    DimensionMismatch {
        shape_a: Vec<usize>,
        shape_b: Vec<usize>,
        op: String,
    },
    #[snafu(display("Singular matrices are not invertible (determinant = {})", determinant))]
    SingularMatrix {
        determinant: usize,
    },
}
