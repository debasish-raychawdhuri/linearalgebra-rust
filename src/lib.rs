/*
 * Copyright  2020 Debasish Ray Chawdhuri
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

extern crate num_bigint as num;
use core::fmt::Debug;

pub mod binary;
pub mod euclidian_domain;
pub mod field;
pub mod int_ring;
use std::ops::*;

pub trait Ring {
    type RingMember: Clone + PartialEq;
    fn add(&self, lhs: &Self::RingMember, rhs: &Self::RingMember) -> Self::RingMember;
    fn mul(&self, lhs: &Self::RingMember, rhs: &Self::RingMember) -> Self::RingMember;
    fn neg(&self, lhs: &Self::RingMember) -> Self::RingMember;
    fn zero(&self) -> Self::RingMember;
    fn one(&self) -> Self::RingMember;
}

pub trait Field: Ring {
    type InvZeroError: Debug;
    fn inv(&self, value: &Self::RingMember) -> Result<Self::RingMember, Self::InvZeroError>;
}

#[derive(PartialEq, Clone, Debug)]
pub struct Matrix<'a, F: Ring> {
    ring: &'a F,
    rows: usize,
    columns: usize,
    data: Vec<Vec<F::RingMember>>,
}

impl<'a, F: Ring> Matrix<'a, F> {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn value_at(&self, row: usize, col: usize) -> F::RingMember {
        self.data[row][col].clone()
    }
}

impl<'a, F: Field> Matrix<'a, F> {
    fn swap_rows(
        &self,
        data1: &mut Vec<Vec<F::RingMember>>,
        data2: &mut Vec<Vec<F::RingMember>>,
        r1: usize,
        r2: usize,
    ) {
        if r1 == r2 {
            return;
        }
        data1.swap(r1, r2);
        data2.swap(r1, r2);
    }
    fn add_multiple_of(
        &self,
        data1: &mut Vec<Vec<F::RingMember>>,
        data2: &mut Vec<Vec<F::RingMember>>,
        r1: usize,
        r2: usize,
        mul: F::RingMember,
    ) {
        for i in 0..data1[r1].len() {
            data1[r1][i] = self
                .ring
                .add(&data1[r1][i], &self.ring.mul(&mul, &data1[r2][i]));
            data2[r1][i] = self
                .ring
                .add(&data2[r1][i], &self.ring.mul(&mul, &data2[r2][i]));
        }
    }
    fn scale_row(
        &self,
        data1: &mut Vec<Vec<F::RingMember>>,
        data2: &mut Vec<Vec<F::RingMember>>,
        r1: usize,
        mul: F::RingMember,
    ) {
        for i in 0..data1[r1].len() {
            data1[r1][i] = self.ring.mul(&mul, &data1[r1][i]);
            data2[r1][i] = self.ring.mul(&mul, &data2[r1][i]);
        }
    }

    fn find_non_zero_pivot(
        &self,
        data1: &Vec<Vec<F::RingMember>>,
        start: usize,
    ) -> Result<usize, String> {
        for i in start..self.rows {
            if data1[i][start] != self.ring.zero() {
                return Ok(i);
            }
        }
        return Err(String::from("No non-zero pivot found"));
    }

    pub fn inverse(&self) -> Result<Self, String> {
        if self.rows != self.columns {
            return Err(String::from("Trying to invert a rectangular matrix"));
        }
        let mut data1 = self.data.clone();
        let mut data2 = Self::one(&self.ring, self.rows).data;

        //triangulation of the matrix. make it an upper triangular matrix
        for i in 0..self.rows {
            let pivot = self.find_non_zero_pivot(&data1, i);
            match pivot {
                Ok(p) => {
                    self.swap_rows(&mut data1, &mut data2, i, p);
                }
                Err(x) => {
                    return Err(x);
                }
            }

            let d = data1[i][i].clone();
            match self.ring.inv(&d) {
                Ok(d_inv) => {
                    self.scale_row(&mut data1, &mut data2, i, d_inv);
                }
                Err(_) => {
                    return Err(String::from("Trying to invert a non-invertible matrix"));
                }
            }
            for j in i + 1..self.rows {
                let mult = self.ring.neg(&data1[j][i]);
                self.add_multiple_of(&mut data1, &mut data2, j, i, mult);
            }
        }

        //Now we make it a identity matrix. Notice that all diagonal entries are already 1

        for i in 0..self.rows {
            for j in i + 1..self.rows {
                let x = self.rows - i - 1;
                let y = self.rows - j - 1;
                let mult = self.ring.neg(&data1[y][x].clone());
                self.add_multiple_of(&mut data1, &mut data2, y, x, mult);
            }
        }

        return Ok(Matrix::new(self.ring, data2));
    }
}

impl<'a, F: Ring> Matrix<'a, F> {
    pub fn new(ring: &'a F, v: Vec<Vec<F::RingMember>>) -> Self {
        let rows = v.len();
        let columns = v[0].len();
        let data = v;
        Matrix {
            ring: &ring,
            rows,
            columns,
            data,
        }
    }

    pub fn one(ring: &'a F, rows: usize) -> Self {
        let mut data = vec![vec![ring.zero(); rows]; rows];
        for i in 0..rows {
            data[i][i] = ring.one();
        }
        Matrix {
            ring: ring,
            rows,
            columns: rows,
            data,
        }
    }

    pub fn zero(ring: &'a F, rows: usize) -> Self {
        let data = vec![vec![ring.zero(); rows]; rows];
        Matrix {
            ring: ring,
            rows,
            columns: rows,
            data,
        }
    }

    pub fn scale(&self, scalar: F::RingMember) -> Matrix<F> {
        let mut ans: Matrix<F> = Matrix {
            ring: self.ring.clone(),
            rows: self.rows,
            columns: self.columns,
            data: vec![vec![self.ring.zero(); self.columns]; self.rows],
        };
        for i in 0..self.rows {
            for j in 0..self.columns {
                ans.data[i][j] = self.ring.mul(&self.data[i][j], &scalar);
            }
        }
        ans
    }

    pub fn add(&self, rhs: &Matrix<'a, F>) -> Result<Matrix<'a, F>, String> {
        if self.rows != rhs.rows || self.columns != rhs.columns {
            return Result::Err(String::from("Illegal matrix operation"));
        } else {
            let mut ans: Matrix<F> = Matrix {
                ring: self.ring,
                rows: self.rows,
                columns: self.columns,
                data: vec![vec![self.ring.zero(); self.columns]; self.rows],
            };
            for i in 0..self.rows {
                for j in 0..self.columns {
                    ans.data[i][j] = self.ring.add(&self.data[i][j], &rhs.data[i][j]);
                }
            }
            Ok(ans)
        }
    }

    pub fn sub(&self, rhs: &Matrix<F>) -> Result<Matrix<F>, String> {
        if self.rows != rhs.rows || self.columns != rhs.columns {
            return Result::Err(String::from("Illegal matrix operation"));
        } else {
            let mut ans: Matrix<F> = Matrix {
                ring: self.ring.clone(),
                rows: self.rows,
                columns: self.columns,
                data: vec![vec![self.ring.zero(); self.columns]; self.rows],
            };
            for i in 0..self.rows {
                for j in 0..self.columns {
                    ans.data[i][j] = self
                        .ring
                        .add(&self.data[i][j], &self.ring.neg(&rhs.data[i][j]));
                }
            }
            Ok(ans)
        }
    }
    //vanila matrix multiplication
    pub fn mul(&self, rhs: &Matrix<F>) -> Result<Matrix<F>, String> {
        if self.columns != rhs.rows {
            return Result::Err(String::from("Illegal matrix operation"));
        } else {
            let mut ans: Matrix<F> = Matrix {
                ring: self.ring,
                rows: self.rows,
                columns: rhs.columns,
                data: vec![vec![self.ring.zero(); rhs.columns]; self.rows],
            };
            for i in 0..self.rows {
                for j in 0..rhs.columns {
                    for k in 0..self.columns {
                        let prod = self.ring.mul(&self.data[i][k], &rhs.data[k][j]);
                        ans.data[i][j] = self.ring.add(&ans.data[i][j], &prod);
                    }
                }
            }
            Ok(ans)
        }
    }
    pub fn transpose(&self) -> Matrix<'a, F> {
        let rows = self.columns;
        let columns = self.rows;
        let mut ans: Matrix<F> = Matrix {
            ring: self.ring.clone(),
            rows: rows,
            columns: columns,
            data: vec![vec![self.ring.zero(); columns]; rows],
        };
        for j in 0..columns {
            for i in 0..rows {
                ans.data[i][j] = self.data[j][i].clone();
            }
        }
        return ans;
    }
}

//Ops implemetations
impl<'a, F: Ring> Add<&Matrix<'a, F>> for &'a Matrix<'a, F> {
    type Output = Matrix<'a, F>;

    fn add(self, rhs: &Matrix<'a, F>) -> Matrix<'a, F> {
        self.add(rhs).expect("")
    }
}

impl<'a, F: Ring> Sub<&Matrix<'a, F>> for &'a Matrix<'a, F> {
    type Output = Matrix<'a, F>;

    fn sub(self, rhs: &Matrix<'a, F>) -> Matrix<'a, F> {
        self.sub(rhs).expect("")
    }
}

impl<'a, F: Ring> Mul<&Matrix<'a, F>> for &'a Matrix<'a, F> {
    type Output = Matrix<'a, F>;

    fn mul(self, rhs: &Matrix<'a, F>) -> Matrix<'a, F> {
        self.mul(rhs).expect("")
    }
}

impl<'a, F: Field> Div<&Matrix<'a, F>> for &'a Matrix<'a, F> {
    type Output = Matrix<'a, F>;

    fn div(self, rhs: &Matrix<F>) -> Matrix<'a, F> {
        self.mul(&rhs.inverse().expect("")).expect("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::ModularField;
    use int_ring::I32Ring;
    use num::BigUint;
    #[test]
    fn test_zero() {
        let ring = I32Ring {};
        let lhs: Matrix<I32Ring> = Matrix::zero(&ring, 4);
        let res = Matrix::new(&ring, vec![vec![0i32; 4]; 4]);
        assert_eq!(lhs, res);
    }

    #[test]
    fn test_one() {
        let ring = I32Ring {};
        let lhs: Matrix<I32Ring> = Matrix::one(&ring, 4);
        let res = Matrix::new(
            &ring,
            vec![
                vec![1, 0, 0, 0],
                vec![0, 1, 0, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 1],
            ],
        );
        assert_eq!(lhs, res);
    }

    #[test]
    fn test_add() {
        let ring = I32Ring {};
        let lhs: Matrix<I32Ring> = Matrix::new(&ring, vec![vec![1, 2, 5], vec![3, 4, 6]]);
        let rhs: Matrix<I32Ring> = Matrix::new(&ring, vec![vec![2, 3, 7], vec![4, 5, 8]]);
        let exp_res: Matrix<I32Ring> = Matrix::new(&ring, vec![vec![3, 5, 12], vec![7, 9, 14]]);
        let res = &lhs + &rhs;
        assert_eq!(exp_res, res);
    }
    #[test]
    fn test_mul() {
        let ring = I32Ring {};
        let lhs: Matrix<I32Ring> = Matrix::new(&ring, vec![vec![1, 2, 3], vec![3, 4, 5]]);
        let rhs: Matrix<I32Ring> = Matrix::new(&ring, vec![vec![2, 3], vec![4, 5], vec![1, 2]]);
        let exp_res: Matrix<I32Ring> = Matrix::new(&ring, vec![vec![13, 19], vec![27, 39]]);
        let res = &lhs * &rhs;
        assert_eq!(exp_res, res);
    }
    fn adder<'a>(ring: &'a I32Ring) -> Matrix<'a, I32Ring> {
        let lhs: Matrix<I32Ring> = Matrix::new(ring, vec![vec![1, 2, 5], vec![3, 4, 6]]);
        let rhs: Matrix<I32Ring> = Matrix::new(ring, vec![vec![2, 3, 7], vec![4, 5, 8]]);
        lhs.add(&rhs).unwrap()
    }
    fn transposer<'a>(ring: &'a I32Ring) -> Matrix<'a, I32Ring> {
        let v = vec![vec![1, 3, 5], vec![2, 4, 6]];
        Matrix::new(ring, v).transpose()
    }

    #[test]
    fn test_transpose() {
        let ring = I32Ring {};
        let lhs: Matrix<I32Ring> = Matrix::new(&ring, vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
        let exp_res: Matrix<I32Ring> = Matrix::new(&ring, vec![vec![1, 3, 5], vec![2, 4, 6]]);
        let res = lhs.transpose();
        assert_eq!(exp_res, res);
    }

    #[test]
    fn test_inverse_1() {
        let ring = ModularField::new(BigUint::from(7u64));
        let one = BigUint::from(1u64);
        let two = BigUint::from(2u64);
        let mat: Matrix<ModularField> =
            Matrix::new(&ring, vec![vec![one.clone(), two.clone()], vec![two, one]]);
        let inv = mat.inverse().expect("");
        let identity_matrix = Matrix::<ModularField>::one(&ring, 2);
        assert_eq!(identity_matrix, inv.mul(&mat).expect(""));
    }

    #[test]
    fn test_inverse_2() {
        let ring = ModularField::new(BigUint::from(7u64));
        let one = BigUint::from(1u64);
        let two = BigUint::from(2u64);
        let three = BigUint::from(3u64);

        let mat: Matrix<ModularField> = Matrix::new(
            &ring,
            vec![
                vec![one.clone(), two.clone(), three.clone()],
                vec![two.clone(), one.clone(), three.clone()],
                vec![three.clone(), one.clone(), three.clone()],
            ],
        );

        let inv = mat.inverse().expect("");
        let identity_matrix = Matrix::<ModularField>::one(&ring, 3);
        assert_eq!(identity_matrix, inv.mul(&mat).expect(""));
    }

    #[test]
    fn test_inverse_3() {
        let ring = ModularField::new(BigUint::from(7u64));
        let one = BigUint::from(1u64);
        let two = BigUint::from(2u64);
        let three = BigUint::from(3u64);

        let mat: Matrix<ModularField> = Matrix::new(
            &ring,
            vec![
                vec![one.clone(), two.clone(), three.clone()],
                vec![one.clone(), two.clone(), one.clone()],
                vec![three.clone(), one.clone(), three.clone()],
            ],
        );

        let inv = mat.inverse().expect("");
        let identity_matrix = Matrix::<ModularField>::one(&ring, 3);
        assert_eq!(identity_matrix, inv.mul(&mat).expect(""));
    }
}
