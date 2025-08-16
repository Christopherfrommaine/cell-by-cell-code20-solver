use crate::int::*;
use crate::ca::*;
use crate::known::known_reversed;
use log::*;

pub fn is_unique_solution(s: Int, period: usize, shift: usize) -> bool {
    let all = run_all(s, period);
    
    // Smallest during period
    let min_org = all.iter().map(|i| i >> i.trailing_zeros()).min().unwrap_or(0);
    let min_rev = all.iter().map(|i| i.reverse_bits()).map(|i| i >> i.trailing_zeros()).min().unwrap_or(0);
    let min = min_org.min(min_rev);
    if min >> min.trailing_zeros() != s >> s.trailing_zeros() {
        return false;
    }

    // Correct period
    for (i, row) in all.into_iter().enumerate() {
        trace!("{:?}, {:?}, {:?}, {:?}", row == s, row == s << shift, i == period, i == 0);
        if (row == s || row == s << shift) != (i == period || i == 0) {
            return false;
        }
    }

    true
}

pub fn is_periodic(s: Int, o: Int, len: usize, offset: usize) -> bool {
    let periodic_bits_mask: u128 = (one() << (len - offset)) - 1;
    debug!("len: {len}, periodic_bits_mask: 0b{periodic_bits_mask:.8b}");
    let known_bits_are_periodic = periodic_bits_mask & s == periodic_bits_mask & o;

    known_bits_are_periodic
}

pub fn is_solution(s: Int, period: usize, len: usize, offset: usize) -> bool {
    let leading_zeros_mask = ((one() << (len + 1)) - 1) ^ ((one() << (len - offset)) - 1);
    let is_solution = (len > 4 * period) && leading_zeros_mask & s == zero();
    
    let o = run(s, period);
    let idk_it_just_works = o >> o.trailing_zeros() == s >> s.trailing_zeros();
    
    is_solution || idk_it_just_works
}


pub fn max_gap_len_of_n(n: u32, mut s: Int) -> bool {

    if s == zero() {return true;}

    s = s >> s.trailing_zeros();

    while s != zero() {
        let tz = s.trailing_zeros();
        if tz > n { return false; }
        s >>= tz + 1;
    }
    true
}


pub fn is_known(s: Int, period: usize, mask: Int) -> bool {
    let all = run_all(s, period);

    let all: Vec<Int> = all.into_iter().map(|a| mask & a).collect();

    let mut counter = 0;

    for mut pat in known_reversed() {

        let padding = 3;

        pat = pat << padding;

        let len = BITS - pat.leading_zeros();

        let mask = (1 << (len + padding)) - 1;

        // println!("m: {mask:b}\np: {pat:b}");

        for a in all.iter().copied() {
            if a == 0 {return false;}

            if (a >> a.trailing_zeros() << padding) & mask == pat {

                // print_plotted_solution(a, period, 0);

                counter += 1;

                break;
            }
        }
    }

    counter >= 2
}

pub fn is_finished(s: Int, len: usize, offset: usize) -> bool {
    
    let leading_zeros_mask = ((one() << (len - offset - 1)) - 1) ^ ((one() << (len - 1)) - 1);

    let all_leading_bits_are_zero = leading_zeros_mask & s == 0;

    if len == offset + 1 {
        return false;
    }

    all_leading_bits_are_zero
}
