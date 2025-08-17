/// This file just reditects all the functions and constants
/// to one of the other uint files. To change the size of the uint,
/// just change all occourances of 256 to 1024, or 128 to 256, or whatever
/// it is currently to whatever you want it to be.

pub type Int = crate::int_using_u1024::Int;

pub const BITS: usize = crate::int_using_u1024::BITS;

pub fn to_u8(n: Int) -> u8 {
    crate::int_using_u1024::to_u8(n)
}

#[inline(always)]
pub fn one() -> Int {
    crate::int_using_u1024::one()
}

#[inline(always)]
pub fn zero() -> Int {
    crate::int_using_u1024::zero()
}