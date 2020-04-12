use super::Ring;
pub struct DivisionAlgorithmResult<R:Ring>{
    quotient: R::RingMember,
    remainder: R::RingMember,
}

pub trait EuclidianDomain: Ring {
    fn division_algorithm(
        value: &Self::RingMember,
        divisor: &Self::RingMember,
    ) -> DivisionAlgorithmResult<Self>;


}
