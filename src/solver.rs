use crate::handle_solution::*;
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

#[allow(dead_code)]
pub fn solve(p: usize, s: usize) {
    println!("Starting solve with p{p}, s{s}");

    let len_i = 2 * p;
    let n_i = one() << len_i;

    println!("Nodes Searched: {}", solve_dfs(n_i, len_i, p, s));
}

pub fn solve_dfs(n: Int, len: usize, p: usize, s: usize) -> u64 {
    // len is the position (zero-indexed from right to left) of the first possible 1

    // Depth Exceeded
    if len > BITS - 2 * p - 5 {
        eprintln!("DEPTH LIMIT REACHED\n{n}");
        std::process::exit(1);
    }

    // Run the automata
    let mut collected = zero();
    let mut o = n;
    for _ in 0..p {
        o = code20(o);
        collected |= o;
    }
    o = o >> s;

    // Unchangable output bits
    let mask = mask_first_bits(len - 2 * p + 1);
    
    // Check Periodicity
    if n & mask != o & mask {return 1;}

    // Check Gaps (for concatonated solutions)
    if s == 0 && !gap_length_less_than(collected & mask, 2) {return 1;}

    // Tilability check (for infinitely repeatable patterns)
    for pattern_length in 1..((len + 1) / 2) {
        let pattern = n & (mask_first_bits(pattern_length) << (len - pattern_length + 1));

        if pattern == zero() {continue;}
        
        let mut rep = zero();
        for i in 0..((len / pattern_length) + 1) {
            rep |= pattern >> (pattern_length * i)
        }

        if (n ^ rep).count_ones() + 10 < n.count_ones() / 2 {
            return 1;
        }
    }

    // Check for Solution
    if o == n {
        // More expensive full run
        let mut all = Vec::with_capacity(p - 1);
        all.push(n);
        for _ in 0..(p - 1) {
            all.push(code20(all[all.len() - 1]));
        }
    
        // Maximum integer acheived over period
        let min_org = all.iter().min().copied().unwrap_or(zero());
        let min_org = min_org >> min_org.trailing_zeros();
        let min_rev = all.iter().map(|i| i.reverse_bits()).min().unwrap_or(zero());
        let min_rev = min_rev >> min_rev.trailing_zeros();

        let min = min_org.min(min_rev);

        if min != n >> n.trailing_zeros() {return 1;}

        // No subperiodicity
        for row in &all[1..] {
            if min == *row >> row.trailing_zeros() {return 1;}
        }

        handle_found_solution(n, p, s);

        return 1;
    }

    // No checks have eliminated cantidate. Continue search.
    let new_len = len + 1;

    let mut nodes_searched = 1;
    if len > 2 * p + 10  {
        // Basic solve at large depths

        nodes_searched += solve_dfs(n, new_len, p, s);
        nodes_searched += solve_dfs(n | (one() << new_len), new_len, p, s);
    } else {
        // Paralelleized solve at top-level nodes
        let mut ns1 = 0;
        let mut ns2 = 0;
        rayon::join(
            || ns1 = solve_dfs(n, new_len, p, s),
        || ns2 = solve_dfs(n | (one() << new_len), new_len, p, s),
        );

        nodes_searched += ns1 + ns2;
    }

    nodes_searched
    
}