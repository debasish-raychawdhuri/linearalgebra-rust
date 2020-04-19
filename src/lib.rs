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
pub mod euclidian_domain;
pub mod int_ring;
pub mod field;

pub trait Ring: Clone {
    type RingMember: Clone;
    fn add(&self, lhs: &Self::RingMember, rhs: &Self::RingMember) -> Self::RingMember;
    fn mul(&self, lhs: &Self::RingMember, rhs: &Self::RingMember) -> Self::RingMember;
    fn neg(&self, lhs: &Self::RingMember) -> Self::RingMember;
    fn zero(&self) -> Self::RingMember;
    fn one(&self) -> Self::RingMember;
}

pub trait Field: Ring + Clone {
    type InvZeroError;
    fn inv(&self, value: &Self::RingMember) -> Result<Self::RingMember, Self::InvZeroError>;
}

#[derive(PartialEq, Clone, Debug)]
pub struct Matrix<F: Ring> {
    ring: F,
    rows: usize,
    columns: usize,
    data: Vec<Vec<F::RingMember>>,
}

impl <F:Ring> Matrix<F> {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize{
        self.columns
    }

    pub fn value_at(&self, row:usize, col:usize) -> F::RingMember {
        self.data[row][col].clone()
    }
}

impl<'a, F: Ring> Matrix<F> {
    pub fn new(ring: F, v: Vec<Vec<F::RingMember>>) -> Self {
        let rows = v.len();
        let columns = v[0].len();
        let data = v;
        Matrix {
            ring: ring.clone(),
            rows,
            columns,
            data,
        }
    }

    pub fn one(ring: F, rows:usize) -> Self {
        let mut data = vec![vec![ring.zero(); rows]; rows];
        for i in 0..rows {
            data[i][i] = ring.one();
        }
        Matrix {
            ring: ring.clone(),
            rows,
            columns:rows,
            data,
        }
    }

    pub fn zero(ring: F, rows:usize) -> Self {
        let data = vec![vec![ring.zero(); rows]; rows];
        Matrix {
            ring: ring.clone(),
            rows,
            columns:rows,
            data,
        }
    }

    pub fn add(&self, rhs: &Matrix<F>) -> Result<Matrix<F>, String> {
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
                    ans.data[i][j] = self.ring.add(&self.data[i][j], &rhs.data[i][j]);
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
                ring: self.ring.clone(),
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
     pub fn transpose(&'a self) -> Matrix<F> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use int_ring::I32Ring;
    #[test]
    fn test_zero() {
        let ring = I32Ring {};
        let lhs: Matrix<I32Ring> = Matrix::zero(ring.clone(), 4);
        let res =  Matrix::new(ring.clone(), vec![vec![0i32;4];4]);
        assert_eq!(lhs, res);
    }

    #[test]
    fn test_one() {
        let ring = I32Ring {};
        let lhs: Matrix<I32Ring> = Matrix::one(ring.clone(), 4);
        let res =  Matrix::new(ring.clone(), vec![vec![1,0,0,0],vec![0,1,0,0], vec![0,0,1,0], vec![0,0,0,1]]);
        assert_eq!(lhs, res);
    }

    #[test]
    fn test_add() {
        let ring = I32Ring {};
        let lhs: Matrix<I32Ring> = Matrix::new(ring.clone(), vec![vec![1, 2, 5], vec![3, 4, 6]]);
        let rhs: Matrix<I32Ring> = Matrix::new(ring.clone(), vec![vec![2, 3, 7], vec![4, 5, 8]]);
        let exp_res: Matrix<I32Ring> = Matrix::new(ring, vec![vec![3, 5, 12], vec![7, 9, 14]]);
        let res = lhs.add(&rhs).expect("Error");
        assert_eq!(exp_res, res);
    }
    #[test]
    fn test_mul() {
        let ring = I32Ring {};
        let lhs: Matrix<I32Ring> = Matrix::new(ring.clone(), vec![vec![1, 2, 3], vec![3, 4, 5]]);
        let rhs: Matrix<I32Ring> = Matrix::new(ring.clone(), vec![vec![2, 3], vec![4, 5], vec![1,2]]);
        let exp_res: Matrix<I32Ring> = Matrix::new(ring, vec![vec![13,19], vec![27,39]]);
        let res = lhs.mul(&rhs).expect("Error");
        assert_eq!(exp_res, res);
    }

    fn adder(ring: I32Ring) -> Matrix<I32Ring> {
        let lhs: Matrix<I32Ring> = Matrix::new(ring.clone(), vec![vec![1, 2, 5], vec![3, 4, 6]]);
        let rhs: Matrix<I32Ring> = Matrix::new(ring, vec![vec![2, 3, 7], vec![4, 5, 8]]);
        lhs.add(&rhs).unwrap()
    }
    fn transposer(ring: I32Ring) -> Matrix<I32Ring> {
         Matrix::new(ring, vec![vec![1, 3, 5], vec![2, 4, 6]]).transpose()
    }

    #[test]
    fn test_transpose() {
        let ring = I32Ring {};
        let lhs: Matrix<I32Ring> = Matrix::new(ring.clone(), vec![vec![1, 2], vec![3, 4], vec![5,6]]);
        let exp_res: Matrix<I32Ring> = Matrix::new(ring, vec![vec![1, 3, 5], vec![2, 4, 6]]);
        let res = lhs.transpose();
        assert_eq!(exp_res, res);
    }
}
