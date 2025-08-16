use crate::handle_solution;
use crate::int::*;
use crate::ca::*;
use crate::checks::*;

pub fn solve_dfs(s: Int, period: usize, shift: usize, len: usize) {

    if len > 90 {return;}

    let offset = 2 * period;

    // Run it
    let o = run(s, period) << shift;

    if o != 0 && o >> o.trailing_zeros() == s >> s.trailing_zeros() {
        if is_unique_solution(s, period, shift) {
            handle_solution::handle_found_solution(s, period, shift);
        }

        return;
    }

    if !is_periodic(s, o, len, offset) {return;}

    // Is part of known
    let certain_bits_mask = (one() << (len - offset)) - 1;
    if !max_gap_len_of_n(2, run_all(s, period).into_iter().map(|a| a & certain_bits_mask).fold(zero(), |a, b| a | b)) {return;}
    if is_known(s, period, certain_bits_mask) {return;}
    if is_solution(s, period, len, offset) {return;}
    if is_finished(s, len, offset) {return;}

    // Canditate solution should continue
    let new_bit_pos = 1 << len;

    solve_dfs(s, period, shift, len + 1);
    solve_dfs(s | new_bit_pos, period, shift, len + 1);

}


pub fn solve(period: usize, shift: usize) {
    println!("Starting solve with p{period}, s{shift}");

    let len = 2 * period + 1;
    let initial_cantidate = one() << (len - 1);

    solve_dfs(initial_cantidate, period, shift, len);
}
