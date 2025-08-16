use crate::handle_solution::handle_found_solution;
use crate::int::*;

#[inline(always)]
pub fn code20(n: Int) -> Int {
    let a = n << 2u32;
    let b = n << 1u32;
    let c = n;
    let d = n >> 1u32;
    let e = n >> 2u32;
    (a ^ b ^ c ^ d ^ e) ^ (a | b | c | d | e)
}

fn gap_length_less_than(mut n: Int, max: u32) -> bool {
    if n == zero() {return true;}

    n = n >> n.trailing_zeros();

    while n != zero() {
        let tz = n.trailing_zeros();
        if tz > max { return false; }
        n >>= tz + 1;
    }
    true
}

pub fn solve(p: usize, s: usize) {
    println!("Starting solve with p{p}, s{s}");

    let len_i = 2 * p;
    let n_i = one() << len_i;

    solve_dfs(n_i, len_i, p, s);
}

pub fn solve_dfs(n: Int, len: usize, p: usize, s: usize) {
    // Run the automata
    let mut collected = zero();
    let mut o = n;
    for _ in 0..p {
        o = code20(o);
        collected |= o;
    }
    o = o >> s;

    // Unchangable output bits
    let mask = one() << (len - 2 * p + 1) - 1;
    
    // Check Periodicity
    if n & mask != o & mask {return;}

    // Check Gaps (for concatonated solutions)
    if !gap_length_less_than(collected, 3) {return;}

    // Check for Solution
    if o == n {
        // More expensive full run
        let mut all = vec![n];
        for _ in 0..p {
            all.push(code20(all[all.len() - 1]));
        }
    
        // Maximum integer acheived over period
        let max_org = all.iter().map(|i| *i >> i.trailing_zeros()).max().unwrap_or(zero());
        let max_rev = all.iter().map(|i| i.reverse_bits()).map(|i| i >> i.trailing_zeros()).max().unwrap_or(zero());
        let max = max_org.min(max_rev);

        if max >> max.trailing_zeros() != n >> n.trailing_zeros() {return;}

        // No subperiodicity
        for row in &all[1..(all.len() - 2)] {
            if *row >> row.trailing_zeros() == max >> max.trailing_zeros() {return;}
        }

        handle_found_solution(n, p, s);

        return;
    }

    // No checks have eliminated cantidate. Continue search.
    solve_dfs(n, len + 1, p, s);
    solve_dfs(n | (one() << len + 1), len + 1, p, s);
}
