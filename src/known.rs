use std::{collections::HashSet};

use crate::int::*;
use log::*;

fn known_original() -> Vec<Int> {
    vec![
        151, 187, 189, 195, 221, 635, 889, 125231, 595703, 610999, 14871103, 
        16537415, 256296063, 22503642597, 222678959859, 10495070598767, 
        360759087837221, 2197520782601119, 11221488970893447375, 
        142082121178470981231,
    ].into_iter().map(|i| from_u128(i)).collect()
}

pub fn known_reversed() -> Vec<Int> {
    known_original()
        .into_iter()
        .flat_map(|i| [i, i.reverse_bits()].into_iter())
        .map(|i| i >> i.trailing_zeros())
        .collect::<HashSet<Int>>()
        .into_iter()
        .collect()
}

fn subsequences_with_gaps(n: Int) -> Vec<Int> {

    let mut bits = Vec::new();
    let len = BITS - n.leading_zeros();
    for i in (0..len).rev() {
        bits.push(to_u8((n >> i) & 1));
    }

    let mut segments = Vec::new();
    let mut cur = Vec::new();
    let mut zero_count = 0;

    for &b in &bits {
        cur.push(b);
        if b == 0 {
            zero_count += 1;
        } else {
            zero_count = 0;
        }
        if zero_count >= 3 {
            cur.truncate(cur.len() - zero_count);
            if !cur.is_empty() {
                segments.push(cur.clone());
            }
            cur.clear();
            zero_count = 0;
        }
    }
    if !cur.is_empty() {
        segments.push(cur);
    }

    if segments.len() <= 1 {
        return vec![n];
    }

    let mut results = Vec::new();
    for start in 0..segments.len() {
        let mut combined = Vec::new();
        for end in start..segments.len() {
            if !combined.is_empty() {
                // Preserve exactly the zeros in the original gap
                let gap_len = bits.iter()
                    .skip(bits.iter().position(|&x| x == 1).unwrap_or(0))
                    .skip_while(|&&x| x == 1)
                    .take_while(|&&x| x == 0)
                    .count();
                combined.extend(vec![0u8; gap_len]);
            }
            combined.extend(&segments[end]);
            let mut val: Int = 0;
            for &bit in &combined {
                val = (val << 1) | bit as Int;
            }
            results.push(val);
        }
    }

    results
}

pub fn number_known_subsequence(mut n: Int) -> u32 {
    if n == 0 {return 0;}

    n = n >> n.trailing_zeros();

    let mut count = 0;

    // println!("n: {n:b}, {}", subsequences_with_gaps(n).into_iter().map(|s| format!("{s:b}")).collect::<Vec<String>>().join(", "));

    let swg = subsequences_with_gaps(n);
    debug!("swg: {:?}", swg);

    for s in swg {

        eprint!("");

        for k in known_reversed() {

            if s == 0 {return 0;}

            if s >> s.trailing_zeros() == k {
                count += 1;
            }
        }
    }

    count
}
