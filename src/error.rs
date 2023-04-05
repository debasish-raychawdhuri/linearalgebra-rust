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
    DimensionMismatchForMatrixAddition(usize, usize, usize, usize),
    DimensionMismatchForMatrixMultiplication(usize, usize, usize, usize),
}

#[derive(Debug, Copy, Clone)]
pub struct Error {
    error_kind: ErrorKind,
}

impl Error {
    pub fn new(error_kind: ErrorKind) -> Self {
        Error { error_kind }
    }
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.error_kind {
            ErrorKind::DivisionByZero => write!(f, "Attempt to divide by zero"),
            ErrorKind::InversionOfZero => write!(f, "Attempt to invert zero"),
            ErrorKind::InversionOfNonInvertibleSquareMatrix => {
                write!(f, "Error trying to invert a non-invertible square matrix")
            }
            ErrorKind::InversionOfRectangularMatrix => {
                write!(f, "Error trying to invert a rectangular matrix")
            }
            ErrorKind::DimensionMismatchForMatrixAddition(rows1, cols1, rows2, cols2) => {
                write!(
                    f,
                    "Error trying to add two matrices of incompatible dimensions: \
                    ({}, {}) and ({}, {})",
                    rows1, cols1, rows2, cols2
                )
            }
            ErrorKind::DimensionMismatchForMatrixMultiplication(rows1, cols1, rows2, cols2) => {
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
