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

use crate::Field;
use crate::Ring;
use crate::euclidian_domain::DivisionAlgorithmResult;
use crate::euclidian_domain::EuclidianDomain;
use funty::Unsigned;
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

pub struct BinaryRing<T: Unsigned> {
    _type_flag: T,
}

struct BitIterator<'a, T: Unsigned> {
    content: &'a [T],
    next_index: usize,
    next_bitmask: T,
    exhausted: bool,
}

impl<'a, T: Unsigned> BitIterator<'a, T> {
    fn new(content: &'a [T]) -> Self {
        BitIterator {
            content,
            next_index: content.len() - 1,
            next_bitmask: T::ONE << (T::BITS - 1),
            exhausted:false,
        }
    }
}

impl<'a, T: Unsigned> Iterator for BitIterator<'a, T> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }
        let result = self.content[self.next_index] & self.next_bitmask > T::ZERO;
        if self.next_bitmask == T::ONE {
            if self.next_index==0{
                self.exhausted =true; 
            }else{
                self.next_index -= 1;
                self.next_bitmask = T::ONE << (T::BITS - 1);
            }
            
        } else {
            self.next_bitmask >>= 1;
        }
        Some(result)
    }
}

impl<T: Unsigned> BinaryRing<T> {
    pub fn shift_left(value: &mut Vec<T>) {
        let top_bit = value[value.len() - 1] & (T::ONE << (T::BITS - 1));
        if top_bit > T::ZERO {
            value.push(T::ZERO);
        }
        let len = value.len();
        for i in (0..len).rev() {
            let top_bit = value[i] & (T::ONE << (T::BITS - 1));
            value[i] = value[i] << 1;
            if i < value.len() - 1 && top_bit > T::ZERO {
                value[i + 1] = value[i + 1] | T::ONE;
            }
        }
    }

    pub fn add_in_place(value: &mut Vec<T>, rhs: &[T]) {
        for _ in value.len()..rhs.len(){
            value.push(T::ZERO);
        }
        for i in 0..rhs.len() {
            value[i] ^= rhs[i];
        }
    }
    pub fn clean_up(value: &mut Vec<T>){
        let len = value.len();
        for i in (0..len).rev() {
            if value[i]!=T::ZERO {
                break;
            }
            value.pop();
        }
    }
}

impl <T:Unsigned>  EuclidianDomain for BinaryRing<T>{
    fn division_algorithm(
        value: &Self::RingMember,
        divisor: &Self::RingMember,
    ) -> DivisionAlgorithmResult<Self> {
        todo!()
    }
}

impl<T: Unsigned> Ring for BinaryRing<T> {
    type RingMember = Vec<T>;

    fn add(&self, lhs: &Self::RingMember, rhs: &Self::RingMember) -> Self::RingMember {
        let left_size = lhs.len();
        let right_size = rhs.len();
        let mut result = Vec::new();
        for i in 0..max(left_size, right_size) {
            let left_val = if i < left_size { lhs[i] } else { T::ZERO };
            let right_val = if i < right_size { rhs[i] } else { T::ZERO };
            result.push(left_val ^ right_val);
        }
        result
    }

    fn mul(&self, lhs: &Self::RingMember, rhs: &Self::RingMember) -> Self::RingMember {
        let left_iter = BitIterator::new(lhs);
        let mut result = vec![T::ZERO];
        for bit in left_iter {
            Self::shift_left(&mut result);
            if bit {
                Self::add_in_place(&mut result, rhs);
            }
        }
        result
    }

    fn neg(&self, lhs: &Self::RingMember) -> Self::RingMember {
        lhs.clone()
    }

    fn zero(&self) -> Self::RingMember {
        vec![T::ZERO]
    }

    fn one(&self) -> Self::RingMember {
        vec![T::ONE]
    }
}

pub struct BinaryField<T: Unsigned> {
    _mod_substractor: T,
}
impl<T: Unsigned> BinaryField<T> {
    pub fn new() -> Self {
        let _mod_substractor = T::ZERO;
        if T::BITS == 8 {
            if let Ok(_mod_substractor) = T::try_from(0b11011) {
                return BinaryField { _mod_substractor };
            }
        } else if T::BITS == 16 {
            if let Ok(_mod_substractor) = T::try_from(0b101011) {
                return BinaryField { _mod_substractor };
            }
        } else if T::BITS == 32 {
            if let Ok(_mod_substractor) = T::try_from(0b10001101) {
                return BinaryField { _mod_substractor };
            }
        } else if T::BITS == 64 {
            if let Ok(_mod_substractor) = T::try_from(0b11011) {
                return BinaryField { _mod_substractor };
            }
        } else if T::BITS == 128 {
            if let Ok(_mod_substractor) = T::try_from(0b10000111) {
                return BinaryField { _mod_substractor };
            }
        }
        return BinaryField {
            _mod_substractor: T::ZERO,
        };
    }
}

impl<T: Unsigned> Ring for BinaryField<T> {
    type RingMember = T;
    fn add(&self, lhs: &Self::RingMember, rhs: &Self::RingMember) -> Self::RingMember {
        *lhs ^ *rhs
    }

    fn mul(&self, lhs: &Self::RingMember, rhs: &Self::RingMember) -> Self::RingMember {
        let mut mul = T::ZERO;
        let rhs = *rhs;
        let mut rhs_musk = T::ONE << (T::BITS - 1);
        for _ in 0..T::BITS {
            let top_bit = mul & (T::ONE << (T::BITS - 1));
            mul = mul << 1;
            if rhs & rhs_musk > T::ZERO {
                mul = mul ^ lhs;
            }
            if top_bit > T::ZERO {
                mul = mul ^ self._mod_substractor;
            }
            rhs_musk = rhs_musk >> 1;
        }
        mul
    }

    fn neg(&self, lhs: &Self::RingMember) -> Self::RingMember {
        *lhs
    }

    fn zero(&self) -> Self::RingMember {
        T::ZERO
    }

    fn one(&self) -> Self::RingMember {
        T::ONE
    }
}

impl<T: Unsigned> Field for BinaryField<T> {
    type InvZeroError = String;

    fn inv(&self, value: &Self::RingMember) -> Result<Self::RingMember, Self::InvZeroError> {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::{BitIterator, BinaryRing};

    #[test]
    fn test_bit_iterator(){
        let value = vec![0x0fu8, 0xffu8];
        let mut iter = BitIterator::new(&value);
        for _ in 0..8{
            assert_eq!(iter.next().unwrap(),true);
        }
        for _ in 0..4{
            assert_eq!(iter.next().unwrap(),false);
        }
        for _ in 0..4{
            assert_eq!(iter.next().unwrap(),true);
        }
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_shif_left(){
        let mut value = vec![0x0fu8, 0xffu8];
        BinaryRing::shift_left(&mut value);
        assert_eq!(value[0], 0x1e);
        assert_eq!(value[1], 0xfe);
        assert_eq!(value[2], 0x01);
    }

    #[test]
    fn test_add_in_place(){
        let mut value = vec![0x0fu8, 0xffu8];
        BinaryRing::add_in_place(&mut value, &vec![0x0fu8]);
        assert_eq!(value[0],0);
        assert_eq!(value[1],0xffu8);
        assert_eq!(value.len(),2);
        BinaryRing::add_in_place(&mut value, &vec![0x0fu8,0xffu8, 0x12u8]);
        assert_eq!(value[0],0x0fu8);
        assert_eq!(value[1],0x00u8);
        assert_eq!(value[2],0x12u8);
        assert_eq!(value.len(),3);

    }
}