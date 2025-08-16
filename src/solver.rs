use crate::handle_solution;
use crate::int::*;
use crate::ca::*;
use log::*;
use rayon::prelude::*;
use crate::checks::*;


fn get_ram_usage_percentage() -> f32 {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_memory();

    let total = sys.total_memory() as f32;
    let used = sys.used_memory() as f32;

    if total == 0.0 {
        0.0
    } else {
        used / total
    }
}


pub fn solve(period: usize, shift: usize) {
    println!("Starting solve with p{period}, s{shift}");

    let offset = 2 * period;

    let mut len = 2 * period + 1;
    let mut searchers: Vec<Int> = vec![1 << (len - 1)];

    while searchers.len() != 0 {
        info!("{}", searchers.len());


        if get_ram_usage_percentage() > 0.7 {assert!(false, "OOM");}


        searchers = searchers.into_par_iter().filter_map(|s| {
            // Run it
            let o = run(s, period) << shift;

            if !is_periodic(s, o, len, offset) {return None;}

            // Is part of known
            let certain_bits_mask = (one() << (len - offset)) - 1;
            if !max_gap_len_of_n(2, run_all(s, period).into_iter().map(|a| a & certain_bits_mask).fold(zero(), |a, b| a | b)) {return None;}
            if is_known(s, period, certain_bits_mask) {return None;}
            

            if is_solution(s, period, len, offset) {

                // Nonzero
                if run(s, 10 * period) == zero() {
                    // assert!(false);
                    return None;
                }

                if is_unique_solution(s, period, shift) {
                    handle_solution::handle_found_solution(s, period, shift);
                }

                return None;
            }

            if is_finished(s, len, offset) {return None;}

            // Automata is a solution so far!
            let new_bit_pos = 1 << len;
            return Some(vec![s, s | new_bit_pos]);

        })
        .collect::<Vec<Vec<Int>>>()
        .into_iter()
        .flat_map(|v| v.into_iter())
        .collect();

        // println!("l{len}, s:\n{}", searchers.iter().map(|s| format!("{s:b}")).collect::<Vec<String>>().join("\n"));


        len += 1;

        
        // assert!(len < 1000);
    }

}