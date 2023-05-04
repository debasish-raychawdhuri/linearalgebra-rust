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
use crate::error::Error;
use crate::Field;
use crate::Ring;
use num::BigUint;
use num::ToBigInt;
use num_bigint::BigInt;
use num_complex::Complex;
use num_integer::Integer;
#[derive(Clone, PartialEq, Debug)]
pub struct ModularField {
    modulus: BigUint,
}

impl ModularField {
    pub fn new(modulus: BigUint) -> ModularField {
        ModularField { modulus }
    }
}

impl Field for ModularField {
    fn inv(&self, x: &BigUint) -> Result<BigUint, Error> {
        if *x == self.zero() {
            return Err(Error::DivisionByZero);
        }
        let signed_x: BigInt = x.to_bigint().unwrap();
        let signed_mod: BigInt = self.modulus.to_bigint().unwrap();

        let ext_gcd = signed_x.extended_gcd(&signed_mod);
        let mut inv = ext_gcd.x % &signed_mod;
        if inv < BigInt::from(0i64) {
            inv += &signed_mod;
        }
        Ok(inv.to_biguint().unwrap())
    }
}

impl Ring for ModularField {
    type RingMember = BigUint;
    fn neg(&self, x: &BigUint) -> BigUint {
        &self.modulus - x
    }
    fn mul(&self, x: &BigUint, y: &BigUint) -> BigUint {
        (x * y) % &self.modulus
    }
    fn add(&self, x: &BigUint, y: &BigUint) -> BigUint {
        (x + y) % &self.modulus
    }

    fn zero(&self) -> BigUint {
        BigUint::from(0u64)
    }
    fn one(&self) -> BigUint {
        BigUint::from(1u64)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct F64Field;

impl Ring for F64Field {
    type RingMember = f64;
    fn mul(&self, x: &f64, y: &f64) -> f64 {
        x * y
    }
    fn add(&self, x: &f64, y: &f64) -> f64 {
        x + y
    }
    fn one(&self) -> f64 {
        1f64
    }
    fn zero(&self) -> f64 {
        0f64
    }
    fn neg(&self, x: &f64) -> f64 {
        -x
    }
}

impl Field for F64Field {
    fn inv(&self, x: &f64) -> Result<f64, Error> {
        if *x == 0f64 {
            Err(Error::DivisionByZero)
        } else {
            Ok(self.one() / x)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ComplexField;
impl Ring for ComplexField {
    type RingMember = Complex<f64>;

    fn mul(&self, x: &Complex<f64>, y: &Complex<f64>) -> Complex<f64> {
        x * y
    }
    fn add(&self, x: &Complex<f64>, y: &Complex<f64>) -> Complex<f64> {
        x + y
    }
    fn one(&self) -> Complex<f64> {
        Complex::<f64>::new(1f64, 0f64)
    }
    fn zero(&self) -> Complex<f64> {
        Complex::<f64>::new(0f64, 0f64)
    }
    fn neg(&self, x: &Complex<f64>) -> Complex<f64> {
        -x
    }
}

impl Field for ComplexField {
    fn inv(&self, x: &Complex<f64>) -> Result<Complex<f64>, Error> {
        if x.re == 0f64 && x.im == 0f64 {
            Err(Error::DivisionByZero)
        } else {
            Ok(self.one() / x)
        }
    }
}
