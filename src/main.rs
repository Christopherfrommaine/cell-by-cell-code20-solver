mod int;
mod handle_solution;
mod ca;
mod solver;
mod known;
mod checks;

use crate::solver::solve;
use crate::handle_solution::{clear_renders, clear_output_file};

fn main() {
    // Initialization
    clear_renders();
    clear_output_file();
    env_logger::init();

    // Run the solver
    solve(6, 0);
}
