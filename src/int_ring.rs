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

use crate::euclidian_domain::DivisionAlgorithmResult;
use crate::euclidian_domain::EuclidianDomain;
use crate::Field;
use crate::Ring;

use itertools::Itertools;
use std::cmp::max;
use std::fmt::Debug;
use std::ops::BitXor;

#[derive(Clone, PartialEq, Debug)]
pub struct I32Ring;

#[derive(Clone, PartialEq, Debug)]
pub struct I64Ring;

impl Ring for I32Ring {
    type RingMember = i32;

    fn zero(&self) -> i32 {
        0i32
    }
    fn neg(&self, v: &i32) -> i32 {
        0 - v
    }
    fn mul(&self, x: &i32, y: &i32) -> i32 {
        x * y
    }
    fn add(&self, x: &i32, y: &i32) -> i32 {
        x + y
    }
    fn one(&self) -> i32 {
        1i32
    }
}

impl Ring for I64Ring {
    type RingMember = i64;

    fn zero(&self) -> i64 {
        0i64
    }
    fn neg(&self, v: &i64) -> i64 {
        0 - v
    }
    fn mul(&self, x: &i64, y: &i64) -> i64 {
        x * y
    }
    fn add(&self, x: &i64, y: &i64) -> i64 {
        x + y
    }
    fn one(&self) -> i64 {
        1i64
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Polynomial<T>
where
    T: Field,
{
    coeffs: Vec<T::RingMember>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct PolynomialRing<'a, T: Field> {
    underlying_field: &'a T,
}

impl<'a, T: Field + PartialEq + Clone> Ring for PolynomialRing<'a, T> {
    type RingMember = Polynomial<T>;

    fn add(&self, lhs: &Self::RingMember, rhs: &Self::RingMember) -> Self::RingMember {
        let sum_coeffs: Vec<T::RingMember> = lhs
            .coeffs
            .iter()
            .zip(rhs.coeffs.iter())
            .map(|(l, r)| self.underlying_field.add(l, r))
            .collect();

        Polynomial { coeffs: sum_coeffs }
    }

    fn mul(&self, lhs: &Self::RingMember, rhs: &Self::RingMember) -> Self::RingMember {
        todo!()
    }

    fn neg(&self, lhs: &Self::RingMember) -> Self::RingMember {
        todo!()
    }

    fn zero(&self) -> Self::RingMember {
        todo!()
    }

    fn one(&self) -> Self::RingMember {
        todo!()
    }
}
