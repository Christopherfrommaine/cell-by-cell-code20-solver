mod int;
mod handle_solution;
mod solver;
mod int_using_u128;
mod int_using_u256;

use crate::solver::*;
use crate::handle_solution::{clear_renders, clear_output_file};

fn main() {
    // Initialization
    clear_renders();
    clear_output_file();

    // Solve for all periods
    for p in 1..20 {
        solve(p, 0);
    }
}
