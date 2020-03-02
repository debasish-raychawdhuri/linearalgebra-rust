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


pub mod int_ring;



pub trait Ring:Clone{
    type RingMember:Clone;
    fn add(lhs: &Self::RingMember, rhs: &Self::RingMember) -> Self::RingMember;
    fn mul(lhs: &Self::RingMember, rhs: &Self::RingMember) -> Self::RingMember;
    fn neg(lhs: &Self::RingMember) -> Self::RingMember;
    fn zero() -> Self::RingMember;
    fn one() -> Self::RingMember;
}

pub trait Field:Ring + Clone {
    type InvZeroError;
    fn inv(value: &Self::RingMember) -> Result<Self::RingMember, Self::InvZeroError>;
}



#[derive(PartialEq, Clone, Debug)]
pub struct  Matrix<F:Ring>{
    rows: usize,
    columns: usize,
    data: Vec<Vec<F::RingMember>>
}

impl <F:Ring> Matrix<F>{
    pub fn new(v: Vec<Vec<F::RingMember>>) -> Self {
        let rows = v.len();
        let columns = v[0].len();
        let data = vec![vec![F::zero();rows];columns];
        Matrix{
            rows,
            columns,
            data,
        }
    }

    pub fn add(&self, rhs:&Matrix<F>) -> Result<Matrix<F>, String> {
        if self.rows != rhs.rows || self.columns != rhs.columns {
            return Result::Err(String::from("Illegal matrix operation"));
        }else{
            let mut ans:Matrix<F> = Matrix{
                rows: self.rows,
                columns: self.columns,
                data: vec![vec![F::zero();self.rows];self.columns]
            };
            for i in 0..self.rows {
                for j in 0..self.columns {
                    ans.data[i][j] = F::add(&self.data[i][j], &rhs.data[i][j]);
                }
            }
            Ok(ans)
        }
    }
    //vanila matrix multiplication
    pub fn mul(&self, rhs:&Matrix<F>) -> Result<Matrix<F>, String> {
        if self.columns != rhs.rows  {
            return Result::Err(String::from("Illegal matrix operation"));
        }else{
            let mut ans:Matrix<F> = Matrix{
                rows: self.rows,
                columns: rhs.columns,
                data: vec![vec![F::zero();self.rows];rhs.columns]
            };
            for i in 0..self.rows {
                for j in 0..rhs.columns {
                    for k in 0..self.columns {
                        let prod = F::mul(&self.data[i][k], &rhs.data[k][j]);
                        ans.data[i][j] = F::add(&ans.data[i][j], &prod);
                    }
                }
            }
            Ok(ans)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use int_ring::I32Ring;
    #[test]
    fn test_add() {
        let lhs: Matrix<I32Ring> = Matrix::new(vec![vec![1,2], vec![3,4]]);
        let rhs: Matrix<I32Ring> = Matrix::new(vec![vec![2,3], vec![4,5]]);
        let exp_res: Matrix<I32Ring> = Matrix::new(vec![vec![3,5], vec![7,9]]);
        let res = lhs.add(&rhs).expect("Error");
        assert_eq!(exp_res, res);
    }
    #[test]
    fn test_mul() {
        let lhs: Matrix<I32Ring> = Matrix::new(vec![vec![1,2], vec![3,4]]);
        let rhs: Matrix<I32Ring> = Matrix::new(vec![vec![2,3], vec![4,5]]);
        let exp_res: Matrix<I32Ring> = Matrix::new(vec![vec![10,13], vec![22,29]]);
        let res = lhs.mul(&rhs).expect("Error");
        assert_eq!(exp_res, res);
    }
}
