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

use std::{cmp::max, marker::PhantomData, vec};

use funty::Unsigned;
use proptest::prelude::*;

use crate::{
    error::Error,
    euclidian_domain::{DivisionAlgorithmResult, EuclidianDomain},
    Field, Ring,
};
pub struct BinaryRing<T: Unsigned> {
    _phantom: PhantomData<T>,
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
            exhausted: false,
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
            if self.next_index == 0 {
                self.exhausted = true;
            } else {
                self.next_index -= 1;
                self.next_bitmask = T::ONE << (T::BITS - 1);
            }
        } else {
            self.next_bitmask >>= 1;
        }
        Some(result)
    }
}

fn bit_length<T: Unsigned>(value: T) -> u32 {
    let mut mask = T::ONE << (T::BITS - 1);
    for i in (1..=T::BITS).rev() {
        if mask & value != T::ZERO {
            return i;
        }
        mask >>= 1;
    }
    0
}

impl<T: Unsigned> Default for BinaryRing<T> {
    fn default() -> Self {
        BinaryRing {
            _phantom: PhantomData::<T>,
        }
    }
}

impl<T: Unsigned> BinaryRing<T> {
    pub fn new() -> Self {
        BinaryRing {
            _phantom: PhantomData::<T>,
        }
    }
    pub fn shift_left_by_bits(value: &mut Vec<T>, bits: u32) {
        if value.is_empty() {
            return;
        }
        let num_of_units_moved = (bits / T::BITS) as usize;
        let bits_within_unit = bits % T::BITS;
        let orig_len = value.len();
        let bits_in_most_sig_unit = bit_length(value[value.len() - 1]);
        if bits_within_unit + bits_in_most_sig_unit <= T::BITS {
            value.resize(value.len() + num_of_units_moved, T::ZERO);
        } else {
            value.resize(value.len() + num_of_units_moved + 1, T::ZERO);
        }
        let cut_mask =
            ((T::ONE << bits_within_unit) - T::ONE) << ((T::BITS - bits_within_unit) % T::BITS);
        for i in (0..orig_len).rev() {
            let cut_val = value[i] & cut_mask;
            value[i + num_of_units_moved] = value[i] << bits_within_unit;
            if i + num_of_units_moved < value.len() - 1 {
                value[i + num_of_units_moved + 1] |=
                    cut_val >> ((T::BITS - bits_within_unit) % T::BITS);
            }
        }

        value
            .iter_mut()
            .take(num_of_units_moved)
            .for_each(|x| *x = T::ZERO);
    }

    pub fn shift_left(value: &mut Vec<T>) {
        if value.is_empty() {
            return;
        }
        let top_bit = value[value.len() - 1] & (T::ONE << (T::BITS - 1));
        if top_bit > T::ZERO {
            value.push(T::ZERO);
        }
        let len = value.len();
        for i in (0..len).rev() {
            let top_bit = value[i] & (T::ONE << (T::BITS - 1));
            value[i] <<= 1;
            if i < value.len() - 1 && top_bit > T::ZERO {
                value[i + 1] |= T::ONE;
            }
        }
    }
    pub fn shift_right(value: &mut Vec<T>) {
        for i in 0..value.len() {
            value[i] >>= 1;
            if i < value.len() - 1 {
                let v = value[i + 1];
                value[i] |= (v & T::ONE) << (T::BITS - 1);
            }
        }
    }
    pub fn bit_at(&self, value: &[T], bit: usize) -> bool {
        let num_of_units = bit / T::BITS as usize;
        let num_of_bits_in_unit = bit % T::BITS as usize;
        let mask = T::ONE << num_of_bits_in_unit;
        value[num_of_units] & mask != T::ZERO
    }

    pub fn degree(&self, value: &Vec<T>) -> i32 {
        let len = value.len();
        if len == 0 {
            return -1;
        }
        let mut clen = 0;
        for i in (0..len).rev() {
            if value[i] != T::ZERO {
                clen = i;
                break;
            }
        }

        let mut mask = T::ONE << (T::BITS - 1);
        let mut bits_in_unit = 0;
        for i in (0..T::BITS).rev() {
            if value[clen] & mask != T::ZERO {
                bits_in_unit = i;
                break;
            }
            mask >>= 1;
        }
        (clen as u32 * T::BITS + bits_in_unit) as i32
    }

    pub fn add_in_place(value: &mut Vec<T>, rhs: &[T]) {
        for _ in value.len()..rhs.len() {
            value.push(T::ZERO);
        }
        for i in 0..rhs.len() {
            value[i] ^= rhs[i];
        }
    }
    pub fn clean_up(value: &mut Vec<T>) {
        let len = value.len();
        for i in (0..len).rev() {
            if value[i] != T::ZERO {
                break;
            }
            value.pop();
        }
    }
}

impl<T: Unsigned> EuclidianDomain for BinaryRing<T> {
    fn division_algorithm(
        &self,
        value: &Self::RingMember,
        divisor: &Self::RingMember,
    ) -> Result<DivisionAlgorithmResult<Self::RingMember>, Error> {
        let v_deg = self.degree(value);
        let d_deg = self.degree(divisor);

        let mut substractor = divisor.clone();
        BinaryRing::clean_up(&mut substractor);

        if substractor == self.zero() {
            return Err(Error::DivisionByZero);
        }

        let mut value = value.clone();
        if v_deg < d_deg {
            Ok(DivisionAlgorithmResult {
                quotient: vec![],
                remainder: value,
            })
        } else {
            Self::shift_left_by_bits(&mut substractor, (v_deg - d_deg) as u32);
            let mut result = vec![T::ZERO];
            for i in (d_deg..=v_deg).rev() {
                BinaryRing::shift_left(&mut result);
                if self.bit_at(&value, i as usize) {
                    BinaryRing::add_in_place(&mut value, &substractor);
                    BinaryRing::add_in_place(&mut result, &[T::ONE]);
                }
                BinaryRing::shift_right(&mut substractor);
            }

            Self::clean_up(&mut result);
            Self::clean_up(&mut value);
            Ok(DivisionAlgorithmResult {
                quotient: result,
                remainder: value,
            })
        }
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
        Self::clean_up(&mut result);
        result
    }

    fn mul(&self, lhs: &Self::RingMember, rhs: &Self::RingMember) -> Self::RingMember {
        if lhs.is_empty() || rhs.is_empty() {
            return vec![];
        }
        let left_iter = BitIterator::new(lhs);
        let mut result = vec![T::ZERO];
        left_iter.for_each(|bit| {
            Self::shift_left(&mut result);
            if bit {
                Self::add_in_place(&mut result, rhs);
            }
        });
        Self::clean_up(&mut result);
        result
    }

    fn neg(&self, lhs: &Self::RingMember) -> Self::RingMember {
        lhs.clone()
    }

    fn zero(&self) -> Self::RingMember {
        vec![]
    }

    fn one(&self) -> Self::RingMember {
        vec![T::ONE]
    }
}

pub struct BinaryField<T: Unsigned> {
    _mod_substractor: T,
}

macro_rules! impl_binary_field_default {
    ($t:ty) => {
        impl Default for BinaryField<$t> {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}
impl_binary_field_default!(u8);
impl_binary_field_default!(u16);
impl_binary_field_default!(u32);
impl_binary_field_default!(u64);
impl_binary_field_default!(u128);

pub trait Newable {
    fn new() -> Self;
}

impl Newable for BinaryField<u8> {
    fn new() -> Self {
        BinaryField::new_with_mod_substractor(0b11011)
    }
}

impl Newable for BinaryField<u16> {
    fn new() -> Self {
        BinaryField::new_with_mod_substractor(0b101011)
    }
}

impl Newable for BinaryField<u32> {
    fn new() -> Self {
        BinaryField::new_with_mod_substractor(0b10001101)
    }
}

impl Newable for BinaryField<u64> {
    fn new() -> Self {
        BinaryField::new_with_mod_substractor(0b11011)
    }
}
impl Newable for BinaryField<u128> {
    fn new() -> Self {
        BinaryField::new_with_mod_substractor(0b10000111)
    }
}

impl<T: Unsigned> BinaryField<T> {
    pub fn new_with_mod_substractor(mod_substractor: T) -> Self {
        BinaryField {
            _mod_substractor: mod_substractor,
        }
    }

    #[allow(unused)]
    fn degree(value: &T) -> u32 {
        let mut mask = T::ONE << (T::BITS - 1);
        let mut deg = 0;
        for i in (0..T::BITS).rev() {
            let bit_set = ((*value & mask) != T::ZERO && deg == 0) as u32;
            deg = bit_set * i + (1 - bit_set) * deg;
            mask >>= 1;
        }
        deg
    }
    fn bit_at(&self, value: &T, index: u32) -> bool {
        let mask = T::ONE << index;
        *value & mask != T::ZERO
    }

    pub fn exponentiate(&self, value: &T, exponent: &T) -> T {
        let mut exp = T::ONE;
        let full_shift = T::BITS - 1;
        let top_bit_mask = T::ONE << full_shift;
        let mut exponent = *exponent;
        for _ in 0..T::BITS {
            let bit = (exponent & top_bit_mask) >> full_shift;
            exp = self.mul(&exp, &exp);
            exp = self.mul(&exp, &(bit * value + (T::ONE - bit) * T::ONE));

            exponent = (exponent ^ top_bit_mask) << 1;
        }
        exp
    }

    #[allow(unused)]
    fn divide_modulus_by_divisor(&self, divisor: T) -> DivisionAlgorithmResult<T> {
        let v_deg = T::BITS;
        let d_deg: u32 = Self::degree(&divisor);
        let mut value = self._mod_substractor;
        let mut substractor = divisor;

        substractor <<= (v_deg - d_deg - 1) as usize;

        value ^= ((substractor & ((T::MAX ^ T::ONE) >> 1)) << 1);

        let mut result = T::ONE;
        for i in (d_deg..v_deg).rev() {
            result <<= 1;
            if self.bit_at(&value, i) {
                value ^= substractor;
                result ^= T::ONE;
            }
            substractor >>= 1;
        }

        DivisionAlgorithmResult {
            quotient: result,
            remainder: value,
        }
    }

    #[allow(unused)]
    fn division_algorithm(&self, value: &T, divisor: &T) -> DivisionAlgorithmResult<T> {
        let v_deg = Self::degree(value);
        let d_deg: u32 = Self::degree(divisor);
        let mut value = *value;
        if v_deg < d_deg {
            DivisionAlgorithmResult {
                quotient: T::ZERO,
                remainder: value,
            }
        } else {
            let mut substractor = *divisor;
            substractor <<= (v_deg - d_deg) as usize;
            let mut result = T::ZERO;
            for i in (d_deg..=v_deg).rev() {
                result <<= 1;
                if self.bit_at(&value, i) {
                    value ^= substractor;
                    result ^= T::ONE;
                }
                substractor >>= 1;
            }

            DivisionAlgorithmResult {
                quotient: result,
                remainder: value,
            }
        }
    }

    #[allow(unused)]
    fn extended_euclid_inv(&self, a: &T) -> T {
        if *a == T::ONE {
            return T::ONE;
        }

        let mut cur = (self.one(), self.zero());
        let mut prev = (self.zero(), self.one());
        let mut cur_divisor = self.add(a, &self.zero());

        let div_result = self.divide_modulus_by_divisor(cur_divisor);
        let mut cur_dividend = cur_divisor;
        cur_divisor = div_result.remainder;
        let temp = prev;
        prev = cur;
        cur = (
            self.add(&temp.0, &self.mul(&prev.0, &self.neg(&div_result.quotient))),
            self.add(&temp.1, &self.mul(&prev.1, &self.neg(&div_result.quotient))),
        );

        loop {
            let div_result = self.division_algorithm(&cur_dividend, &cur_divisor);
            if div_result.remainder == T::ZERO {
                return cur.0;
            }

            cur_dividend = cur_divisor;
            cur_divisor = div_result.remainder;
            let temp = prev;
            prev = cur;
            cur = (
                self.add(&temp.0, &self.mul(&prev.0, &self.neg(&div_result.quotient))),
                self.add(&temp.1, &self.mul(&prev.1, &self.neg(&div_result.quotient))),
            );
        }
    }
    pub fn gcd_inv(&self, value: &T) -> Result<T, Error> {
        if *value == T::ZERO {
            return Err(Error::InversionOfZero);
        }
        Ok(self.extended_euclid_inv(value))
    }
}

impl<T: Unsigned> Ring for BinaryField<T> {
    type RingMember = T;
    fn add(&self, lhs: &Self::RingMember, rhs: &Self::RingMember) -> Self::RingMember {
        *lhs ^ *rhs
    }

    fn mul(&self, lhs: &Self::RingMember, rhs: &Self::RingMember) -> Self::RingMember {
        let mut mul = T::ZERO;
        let mut rhs = *rhs;
        let full_shift = T::BITS - 1;
        for _ in 0..T::BITS {
            let top_bit = mul >> full_shift;
            mul <<= 1;

            let b = rhs >> full_shift;
            mul ^= b * lhs;
            mul ^= top_bit * self._mod_substractor;
            rhs <<= 1;
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
    fn inv(&self, value: &T) -> Result<T, Error> {
        if *value == T::ZERO {
            return Err(crate::error::Error::DivisionByZero);
        }
        let mut exp = T::ONE;
        for _ in 0..T::BITS - 1 {
            exp = self.mul(&exp, &exp);
            exp = self.mul(&exp, value);
        }
        exp = self.mul(&exp, &exp);
        Ok(exp)
    }
}

#[cfg(test)]
mod tests {
    use crate::Ring;

    use super::{BinaryRing, BitIterator};

    #[test]
    fn test_bit_iterator() {
        let value = vec![0x0fu8, 0xffu8];
        let mut iter = BitIterator::new(&value);
        for _ in 0..8 {
            assert!(iter.next().unwrap());
        }
        for _ in 0..4 {
            assert!(!iter.next().unwrap());
        }
        for _ in 0..4 {
            assert!(iter.next().unwrap());
        }
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_shif_left() {
        let mut value = vec![0x0fu8, 0xffu8];
        BinaryRing::shift_left(&mut value);
        assert_eq!(value[0], 0x1e);
        assert_eq!(value[1], 0xfe);
        assert_eq!(value[2], 0x01);
    }

    #[test]
    fn test_binary_add_in_place() {
        let mut value = vec![0x0fu8, 0xffu8];
        BinaryRing::add_in_place(&mut value, &[0x0fu8]);
        assert_eq!(value[0], 0);
        assert_eq!(value[1], 0xffu8);
        assert_eq!(value.len(), 2);
        BinaryRing::add_in_place(&mut value, &[0x0fu8, 0xffu8, 0x12u8]);
        assert_eq!(value[0], 0x0fu8);
        assert_eq!(value[1], 0x00u8);
        assert_eq!(value[2], 0x12u8);
        assert_eq!(value.len(), 3);
    }
    #[test]
    fn test_bindary_ring_mult() {
        let v1 = vec![0x0fu8, 0xffu8];
        let v2 = vec![0x0f];
        let ring = BinaryRing::new();
        let result = ring.mul(&v1, &v2);
        assert_eq!(result[0], 0x55);
        assert_eq!(result[1], 0x05);
        assert_eq!(result[2], 0x05);
    }
    #[test]
    fn test_binary_ring_deg() {
        let v1 = vec![0x0fu8, 0x4fu8];
        let ring = BinaryRing::new();
        assert_eq!(ring.degree(&v1), 14);
    }
    #[test]
    fn test_shift_by_bits() {
        let mut vi = vec![0x2f, 0x1fu8];

        BinaryRing::shift_left_by_bits(&mut vi, 12);
        assert_eq!(vi[0], 0x00);
        assert_eq!(vi[1], 0xf0);
        assert_eq!(vi[2], 0xf2);
        assert_eq!(vi[3], 0x01);
    }
}

proptest! {

    #[test]
    fn test_shifts(v:Vec<u8>) {
        let mut w=v.clone();
        let mut u=v;
        BinaryRing::clean_up(&mut u);
        BinaryRing::shift_left(&mut w);
        BinaryRing::shift_right(&mut w);

        BinaryRing::clean_up(&mut w);
        assert_eq!(u,w);
    }
    #[test]
    fn test_shifts_long(v:Vec<u8>, count in 0..32u32) {
        let mut w=v.clone();
        let mut u=v;
        BinaryRing::clean_up(&mut u);
        BinaryRing::shift_left_by_bits(&mut w, count);
        for _ in 0..count{
            BinaryRing::shift_right(&mut w);
        }

        BinaryRing::clean_up(&mut w);
        assert_eq!(u,w);
    }
    #[test]
    fn test_mul_div(a:Vec<u8>,b:Vec<u8>){
        let mut a = a;
        let mut b = b;
        BinaryRing::clean_up(&mut a);
        BinaryRing::clean_up(&mut b);
        if b.is_empty()  && a.is_empty(){
            if a.len() > b.len() {
               std::mem::swap(&mut a, &mut b);
            }
            let ring = BinaryRing::new();
            let div_result = ring.division_algorithm(&b,&a).unwrap();
            let mut mul_result = ring.add(&ring.mul(&div_result.quotient, &a), &div_result.remainder);
            BinaryRing::clean_up(&mut mul_result);
            assert_eq!(b, mul_result);
        }
    }
     #[test]
    fn test_mul_div_field(a:u8,b:u8){
        let mut a = a;
        let mut b = b;
        if b!=0 {
           if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            let ring = BinaryField::new();
            let div_result = ring.division_algorithm(&b,&a);
            let mul_result = ring.add(&ring.mul(&div_result.quotient, &a), &div_result.remainder);
            assert_eq!(b, mul_result);
        }
    }
    #[test]
    fn test_mul_div_mod(a:u8){
        let a = a;
        if a>1 {
            let field = BinaryField::new();
            let div_result = field.divide_modulus_by_divisor(a);
            let mul_result = field.add(&field.mul(&div_result.quotient, &a), &div_result.remainder);
            assert_eq!(mul_result,0);
        }
    }
    #[test]
    fn test_field_inverse_u8(a:u8){
        if a >=1 {
            let field = BinaryField::new();
            let inv = field.inv(&a).unwrap();
            let prod = field.mul(&a,&inv);
            assert_eq!(prod, 1);
        }
    }
    #[test]
    fn test_field_inverse_u16(a:u16){
        if a != 0 {
            let field = BinaryField::new();
            let inv = field.inv(&a).unwrap();
            let prod = field.mul(&a,&inv);
            assert_eq!(prod, 1);
        }
    }
   #[test]
    fn test_field_inverse_u32(a:u32){
        if a != 0 {
            let field = BinaryField::new();
            let inv = field.inv(&a).unwrap();
            let prod = field.mul(&a,&inv);
            assert_eq!(prod, 1);
        }
    }
   #[test]
    fn test_field_inverse_u64(a:u64){
        if a != 0 {
            let field = BinaryField::new();
            let inv = field.inv(&a).unwrap();
            let prod = field.mul(&a,&inv);
            assert_eq!(prod, 1);
        }
    }
   #[test]
    fn test_field_inverse_u128(a:u128){
        if a != 0 {
            let field = BinaryField::new();
            let inv = field.inv(&a).unwrap();
            let prod = field.mul(&a,&inv);
            assert_eq!(prod, 1);
        }
    }

    #[test]
    fn test_field_inverse_gcd(a:u8){
        if a != 0 {
            let field = BinaryField::new();
            let inv = field.inv(&a).unwrap();
            let prod = field.mul(&a,&inv);
            assert_eq!(prod, 1);
        }
    }
    #[test]
    fn test_field_inverse_gcd_u16(a:u16){
        if a != 0 {
            let field = BinaryField::new();
            let inv = field.inv(&a).unwrap();
            let prod = field.mul(&a,&inv);
            assert_eq!(prod, 1);
        }
    }
    #[test]
    fn test_field_inverse_gcd_u32(a:u32){
        if a != 0 {
            let field = BinaryField::new();
            let inv = field.inv(&a).unwrap();
            let prod = field.mul(&a,&inv);
            assert_eq!(prod, 1);
        }
    }
    #[test]
    fn test_field_inverse_gcd_u64(a:u64){
        if a != 0 {
            let field = BinaryField::new();
            let inv = field.inv(&a).unwrap();
            let prod = field.mul(&a,&inv);
            assert_eq!(prod, 1);
        }
    }
    #[test]
    fn test_field_inverse_gcd_u128(a:u128){
        if a != 0 {
            let field = BinaryField::new();
            let inv = field.inv(&a).unwrap();
            let prod = field.mul(&a,&inv);
            assert_eq!(prod, 1);
        }
    }

}
