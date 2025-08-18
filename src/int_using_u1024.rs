use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr, ShrAssign};

#[derive(Clone, Copy, Default)]
pub struct U1024 {
    v: [u64; 16],
}


#[allow(dead_code)]
pub type Int = U1024;


#[allow(dead_code)]
pub const BITS: usize = 1024;


#[allow(dead_code)]
pub fn to_u8(n: Int) -> u8 {
    n.v[0] as u8
}


#[allow(dead_code)]
#[inline(always)]
pub fn one() -> Int {
    U1024 { v: [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }
}


#[allow(dead_code)]
pub fn from_u128(n: u128) -> U1024 {
    let w1 = n as u64;
    let w2 = (n >> 64) as u64;

    U1024 { v: [w1, w2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }
}

#[allow(dead_code)]
pub fn to_u128(n: Int) -> u128 {
    debug_assert!(n.v[2..].iter().copied().all(|i| i == 0));
    n.v[0] as u128 + ((n.v[1] as u128) << 64)
}

#[allow(dead_code)]
#[inline(always)]
pub fn zero() -> Int {
    U1024 { v: [0; 16] }
}

impl U1024 {
    pub fn from_words(words: [u64; 16]) -> Self {
        Self { v: words }
    }
}

use std::cmp::Ordering;

impl PartialEq for U1024 {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v
    }
}
impl Eq for U1024 {}

impl PartialOrd for U1024 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for U1024 {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare limbs from most-significant to least-significant
        for i in (0..16).rev() {
            if self.v[i] < other.v[i] { return Ordering::Less; }
            if self.v[i] > other.v[i] { return Ordering::Greater; }
        }
        Ordering::Equal
    }
}

use std::fmt;
impl fmt::Display for U1024 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.v.iter().all(|&x| x == 0) { return write!(f, "0"); }

        let mut n = *self;
        let mut digits = Vec::new();

        while n.v.iter().any(|&x| x != 0) {
            let mut remainder = 0u64;
            for i in (0..16).rev() {
                let value = ((remainder as u128) << 64) | (n.v[i] as u128);
                n.v[i] = (value / 10) as u64;
                remainder = (value % 10) as u64;
            }
            digits.push(remainder as u8);
        }

        for d in digits.iter().rev() { write!(f, "{}", d)?; }
        Ok(())
    }
}
impl fmt::Debug for Int {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl BitAnd for U1024 {
    type Output = U1024;
    fn bitand(self, rhs: U1024) -> U1024 {
        let mut out = [0u64; 16];
        for i in 0..16 { out[i] = self.v[i] & rhs.v[i]; }
        U1024::from_words(out)
    }
}
impl BitOr for U1024 {
    type Output = U1024;
    fn bitor(self, rhs: U1024) -> U1024 {
        let mut out = [0u64; 16];
        for i in 0..16 { out[i] = self.v[i] | rhs.v[i]; }
        U1024::from_words(out)
    }
}
impl BitXor for U1024 {
    type Output = U1024;
    fn bitxor(self, rhs: U1024) -> U1024 {
        let mut out = [0u64; 16];
        for i in 0..16 { out[i] = self.v[i] ^ rhs.v[i]; }
        U1024::from_words(out)
    }
}
impl Not for U1024 {
    type Output = U1024;
    fn not(self) -> U1024 {
        let mut out = [0u64; 16];
        for i in 0..16 { out[i] = !self.v[i]; }
        U1024::from_words(out)
    }
}

/* Assign variants */
impl BitAndAssign for U1024 { fn bitand_assign(&mut self, rhs: U1024) { for i in 0..16 { self.v[i] &= rhs.v[i]; } } }
impl BitOrAssign for U1024 { fn bitor_assign(&mut self, rhs: U1024) { for i in 0..16 { self.v[i] |= rhs.v[i]; } } }
impl BitXorAssign for U1024 { fn bitxor_assign(&mut self, rhs: U1024) { for i in 0..16 { self.v[i] ^= rhs.v[i]; } } }

/* Shifts */
fn shl_words(src: &[u64; 16], n: usize) -> [u64; 16] {
    if n >= 1024 { return [0; 16]; }
    let word_shift = n / 64;
    let bit_shift = n % 64;
    let mut out = [0u64; 16];

    if bit_shift == 0 {
        for i in (word_shift..16).rev() { out[i] = src[i - word_shift]; }
        return out;
    }

    for i in (0..16).rev() {
        let src_idx = i as isize - word_shift as isize;
        if src_idx < 0 { out[i] = 0; } 
        else {
            let lo = src[src_idx as usize] << bit_shift;
            let hi = if src_idx as usize >= 1 { src[src_idx as usize - 1] >> (64 - bit_shift) } else { 0 };
            out[i] = lo | hi;
        }
    }
    out
}

fn shr_words(src: &[u64; 16], n: usize) -> [u64; 16] {
    if n >= 1024 { return [0; 16]; }
    let word_shift = n / 64;
    let bit_shift = n % 64;
    let mut out = [0u64; 16];

    if bit_shift == 0 {
        for i in 0..(16 - word_shift) { out[i] = src[i + word_shift]; }
        return out;
    }

    for i in 0..16 {
        let src_idx = i + word_shift;
        if src_idx >= 16 { out[i] = 0; }
        else {
            let lo = src[src_idx] >> bit_shift;
            let hi = if src_idx + 1 < 16 { src[src_idx + 1] << (64 - bit_shift) } else { 0 };
            out[i] = lo | hi;
        }
    }
    out
}

impl Shl<usize> for U1024 { type Output = U1024; fn shl(self, rhs: usize) -> U1024 { U1024::from_words(shl_words(&self.v, rhs)) } }
impl Shr<usize> for U1024 { type Output = U1024; fn shr(self, rhs: usize) -> U1024 { U1024::from_words(shr_words(&self.v, rhs)) } }
impl ShlAssign<usize> for U1024 { fn shl_assign(&mut self, rhs: usize) { self.v = shl_words(&self.v, rhs); } }
impl ShrAssign<usize> for U1024 { fn shr_assign(&mut self, rhs: usize) { self.v = shr_words(&self.v, rhs); } }

impl Shl<u32> for U1024 { type Output = U1024; fn shl(self, rhs: u32) -> U1024 { U1024::from_words(shl_words(&self.v, rhs as usize)) } }
impl Shr<u32> for U1024 { type Output = U1024; fn shr(self, rhs: u32) -> U1024 { U1024::from_words(shr_words(&self.v, rhs as usize)) } }
impl ShlAssign<u32> for U1024 { fn shl_assign(&mut self, rhs: u32) { self.v = shl_words(&self.v, rhs as usize); } }
impl ShrAssign<u32> for U1024 { fn shr_assign(&mut self, rhs: u32) { self.v = shr_words(&self.v, rhs as usize); } }

/* Reference convenience */
impl<'a> BitAnd for &'a U1024 { type Output = U1024; fn bitand(self, rhs: &'a U1024) -> U1024 { *self & *rhs } }
impl<'a> BitOr for &'a U1024 { type Output = U1024; fn bitor(self, rhs: &'a U1024) -> U1024 { *self | *rhs } }
impl<'a> BitXor for &'a U1024 { type Output = U1024; fn bitxor(self, rhs: &'a U1024) -> U1024 { *self ^ *rhs } }

impl U1024 {
    pub fn trailing_zeros(&self) -> u32 {
        for i in 0..16 { if self.v[i] != 0 { return (i as u32) * 64 + self.v[i].trailing_zeros(); } }
        1024
    }

    pub fn reverse_bits(&self) -> U1024 {
        let mut out = [0u64; 16];
        for i in 0..16 { out[i] = self.v[15 - i].reverse_bits(); }
        U1024::from_words(out)
    }

    pub fn min(self, other: U1024) -> U1024 {
        for i in (0..16).rev() {
            if self.v[i] < other.v[i] { return self; } 
            else if self.v[i] > other.v[i] { return other; }
        }
        self
    }

    pub fn ilog2(&self) -> u32 {
        for i in (0..16).rev() {
            if self.v[i] != 0 { return (i as u32) * 64 + 63 - self.v[i].leading_zeros(); }
        }
        panic!("ilog2 called on zero");
    }
}
