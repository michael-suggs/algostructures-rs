//! Linear Algebra

pub mod matrix;

pub enum MatrixError {
    DimensionMismatchError,
    NotInvertibleError,
}
