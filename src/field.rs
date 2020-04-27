use crate::Field;
use crate::Ring;
use num::BigUint;
use num_complex::Complex;
#[derive(Clone, PartialEq, Debug)]
pub struct ModularField {
    modulus: BigUint,
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
    fn mul(&self, x: &f64, y: &f64) -> f64{x*y}
    fn add(&self, x: &f64, y: &f64) -> f64 { x+y}
    fn one(&self) -> f64 { 1f64 }
    fn zero(&self) -> f64 { 0f64 }
    fn neg(&self, x: &f64) -> f64 { -x }
}

impl  Field for F64Field {
    fn inv(&self, x: &f64) -> Result<f64, String> {
        if *x == 0f64 {
            Err(String::from("Division by zero"))
        }else {
            Ok (self.one() / x)
        }
    }
    type InvZeroError = String;
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ComplexField;
impl Ring for ComplexField {
    type RingMember = Complex<f64>;

    fn mul(&self, x: &Complex<f64>, y: &Complex<f64>) -> Complex<f64>{x*y}
    fn add(&self, x: &Complex<f64>, y: &Complex<f64>) -> Complex<f64> { x+y}
    fn one(&self) ->Complex<f64> { Complex::<f64>::new(1f64,0f64) }
    fn zero(&self) -> Complex<f64> { Complex::<f64>::new(0f64,0f64) }
    fn neg(&self, x: &Complex<f64>) -> Complex<f64> { -x }
}

impl  Field for ComplexField {
    fn inv(&self, x: &Complex<f64>) -> Result<Complex<f64>, String> {
        if x.re == 0f64 && x.im == 0f64 {
            Err(String::from("Division by zero"))
        }else {
            Ok (self.one() / x)
        }
    }
    type InvZeroError = String;
}
