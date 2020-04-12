use crate::Field;
use crate::Ring;
use num::BigUint;
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
