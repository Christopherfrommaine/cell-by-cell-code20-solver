mod int;
mod handle_solution;
mod ca;
mod solver;
mod known;
mod testing;
mod checks;

use crate::solver::solve;
use crate::handle_solution::{clear_renders, clear_output_file};

fn main() {
    clear_renders();
    clear_output_file();
    env_logger::init();

    solve(6, 0);
}
