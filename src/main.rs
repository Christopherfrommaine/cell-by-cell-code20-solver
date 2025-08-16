mod int;
mod handle_solution;
mod solver;

use crate::solver::solve;
use crate::handle_solution::{clear_renders, clear_output_file};

fn main() {
    // Initialization
    clear_renders();
    clear_output_file();
    env_logger::init();

    // Run the solver
    // solve(6, 0);

    solve(10, 0);

    // for period in 1..20 {
    //     solve(period, 0);
    // }
}
