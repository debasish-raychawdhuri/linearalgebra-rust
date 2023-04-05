/*
 * Copyright 2023 Debasish Ray Chawdhuri
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do
 * so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
 * HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
 * WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

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
