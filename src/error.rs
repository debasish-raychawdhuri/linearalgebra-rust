use std::{error::Error as StdError, fmt::Display};

#[derive(Debug, Copy, Clone)]
pub enum ErrorKind {
    DivisionByZero,
    InversionOfZero,
    InversionOfNonInvertibleSquareMatrix,
    InversionOfRectangularMatrix,
    DimensionMismatchForMatrixAddition,
    DimensionMismatchForMatrixMultiplication,
}

#[derive(Debug, Copy, Clone)]
pub struct Error {
    error_kind: ErrorKind,
}

impl Error {
    fn get_description(&self) -> &str {
        match self.error_kind {
            ErrorKind::DivisionByZero => "Attemt to divide by zero",
            ErrorKind::InversionOfZero => "Attempt to invert zero",
            ErrorKind::InversionOfNonInvertibleSquareMatrix => {
                "Error trying to inverst a non-invertible matrix"
            }
            ErrorKind::InversionOfRectangularMatrix => {
                "Error trying to invert a rectangular matrix"
            }
            ErrorKind::DimensionMismatchForMatrixAddition => {
                "Error trying to add two unequal matrices"
            }
            ErrorKind::DimensionMismatchForMatrixMultiplication => {
                "Error trying to multiply two incompatible dimensions"
            }
        }
    }

    pub fn new(error_kind: ErrorKind) -> Self {
        Error { error_kind }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }

    fn description(&self) -> &str {
        self.get_description()
    }

    fn cause(&self) -> Option<&dyn StdError> {
        self.source()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_description())
    }
}
