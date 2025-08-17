pub type Int = crate::int_using_u256::Int;

pub const BITS: usize = crate::int_using_u256::BITS;

pub fn to_u8(n: Int) -> u8 {
    crate::int_using_u256::to_u8(n)
}

#[inline(always)]
pub fn one() -> Int {
    crate::int_using_u256::one()
}

#[inline(always)]
pub fn zero() -> Int {
    crate::int_using_u256::zero()
}