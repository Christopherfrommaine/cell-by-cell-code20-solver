pub mod int;
pub mod handle_solution;
pub mod solver;

// Using a fixed-width number for speed.
// int.rs implements an integer based on the code
// in either of these two files:
pub mod int_using_u128;
pub mod int_using_u256;
pub mod int_using_u1024;

use crate::solver::*;
use crate::handle_solution::{clear_renders, clear_output_file};

fn main() {
    // Initialization
    clear_renders();
    clear_output_file();

    // Fix stack overflow during deep recursion with large integers
    rayon::ThreadPoolBuilder::new()
        .stack_size(32 * 1024 * 1024) // 32 MB stack
        .build_global()
        .unwrap();


    // Solve for all periods
    for p in 1..=10 {
        solve(p, 0);
    }
}
