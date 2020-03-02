use crate::Ring;
#[derive(Clone, PartialEq, Debug)]
pub struct I32Ring;

#[derive(Clone, PartialEq, Debug)]
pub struct I64Ring;

impl Ring for I32Ring{
    type RingMember = i32;

    fn zero() -> <Self as Ring>::RingMember { 0i32 }
    fn neg(v: &i32) -> i32 { 0-v }
    fn mul(x: &i32, y: &i32) -> i32 { x*y }
    fn add(x: &i32, y: &i32) -> i32 { x+y }
    fn one() -> i32 { 1i32 }
}

impl Ring for I64Ring{
    type RingMember = i64;

    fn zero() ->i64 { 0i64 }
    fn neg(v: &i64) -> i64 { 0-v }
    fn mul(x: &i64, y: &i64) -> i64 { x*y }
    fn add(x: &i64, y: &i64) -> i64 { x+y }
    fn one() -> i64 { 1i64 }
}
