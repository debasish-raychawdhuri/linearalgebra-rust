use std::{error::Error as StdError, fmt::Display};

#[derive(Debug)]
pub enum Error {
    DivisionByZero,
    InversionOfZero,
    InversionOfNonInvertibleSquareMatrix,
    InversionOfRectangularMatrix,
    DimensionMismatchForMatrixAddition(usize, usize, usize, usize),
    DimensionMismatchForMatrixMultiplication(usize, usize, usize, usize),
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DivisionByZero => write!(f, "Attempt to divide by zero"),
            Error::InversionOfZero => write!(f, "Attempt to invert zero"),
            Error::InversionOfNonInvertibleSquareMatrix => {
                write!(f, "Error trying to invert a non-invertible square matrix")
            }
            Error::InversionOfRectangularMatrix => {
                write!(f, "Error trying to invert a rectangular matrix")
            }
            Error::DimensionMismatchForMatrixAddition(rows1, cols1, rows2, cols2) => {
                write!(
                    f,
                    "Error trying to add two matrices of incompatible dimensions: \
                    ({}, {}) and ({}, {})",
                    rows1, cols1, rows2, cols2
                )
            }
            Error::DimensionMismatchForMatrixMultiplication(rows1, cols1, rows2, cols2) => {
                write!(
                    f,
                    "Error trying to multiply two matrices of incompatible dimensions: \
                    ({}, {}) and ({}, {})",
                    rows1, cols1, rows2, cols2
                )
            }
        }
    }
}
