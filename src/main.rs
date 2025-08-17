mod int;
mod handle_solution;
mod solver;

// Using a fixed-width number for speed.
// int.rs implements an integer based on the code
// in either of these two files:
// mod int_using_u128;
mod int_using_u256;

use crate::solver::*;
use crate::handle_solution::{clear_renders, clear_output_file};

fn main() {
    // Initialization
    clear_renders();
    clear_output_file();

    // Solve for all periods
    for p in 1..50 {
        solve(p, 0);
    }
}
