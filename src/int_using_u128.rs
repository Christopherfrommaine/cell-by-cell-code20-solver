// Leter, I may need to change this to a custom u256 or u512 type,
// so implement neccesary things here.

use std::u128;

pub type Int = u128;

pub const BITS: usize = 128;

pub fn to_u8(n: Int) -> u8 {
    n as u8
}

#[inline(always)]
pub fn one() -> Int {
    1
}

#[inline(always)]
pub fn zero() -> Int {
    0
}