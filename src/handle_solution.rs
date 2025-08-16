use crate::{ca::{bits, code20, run_all}, int::{zero, Int}};
use std::{fs, path::Path, io::{Write, BufRead, BufReader}};

fn log_solution_to_file(sol: Int, period: usize, shift: usize) {
    let filename = "output.txt";
    let string = format!("n{sol} p{period} s{shift}\n");

    let mut file = match std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)
    {
        Ok(f) => f,
        Err(_) => return,
    };

    if std::io::Write::write_all(&mut file, string.as_bytes()).is_err() {
        return;
    }
}

fn solution_to_array(sol: Int, period: usize, shift: usize) -> Vec<Vec<u8>> {
    let repititions = 10;
    let padding = 10 + shift * repititions;

    let w: usize = sol.ilog2() as usize + 2 * padding;

    let mut state = sol << padding;
    let mut arr = vec![bits(state, w)];
    for _ in 0..(repititions * period) {
        state = code20(state);
        arr.push(bits(state, w));

        if state == zero() {
            break;
        }
    }

    arr
}

#[allow(dead_code)]
pub fn print_plotted_solution(sol: Int, period: usize, shift: usize) {
    let arr = solution_to_array(sol, period, shift);

    use cgrustplot::{plots::array_plot::array_plot};
    array_plot(&arr)
        .set_axes(false)
        .set_title(&format!("p{period} s{shift} n{sol}"))
        .print();
}

fn save_rendered_solution(sol: Int, period: usize, shift: usize) {
    let arr = solution_to_array(sol, period, shift);

    use cgrustplot::{plots::array_plot::array_plot};
    array_plot(&arr)
        .set_axes(false)
        .set_title(&format!("p{period} s{shift} n{sol}"))
        .as_image()
        .save(&format!("renders/solution_p{period}_s{shift}_n{sol}.png"))
}

fn read_tuples_from_file() -> Vec<(u128, usize, usize)> {
    let file = fs::File::open("output.txt").expect("No output.txt file found.");
    let reader = BufReader::new(file);

    let mut tuples = Vec::new();

    for line in reader.lines().flatten() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 3 {
            continue;
        }
        let n = parts[0].trim_start_matches('n').parse::<u128>();
        let p = parts[1].trim_start_matches('p').parse::<usize>();
        let s = parts[2].trim_start_matches('s').parse::<usize>();

        if let (Ok(n), Ok(p), Ok(s)) = (n, p, s) {
            tuples.push((n, p, s));
            tuples.push((n.reverse_bits(), p, s));
        }
    }

    tuples
}

fn is_already_found(n: Int, period: usize, shift: usize) -> bool {
    let found = read_tuples_from_file();

    for row in run_all(n, period) {
        for f in found.iter().copied() {
            if row >> row.trailing_zeros() == f.0 >> f.0.trailing_zeros() {
                return true;
            }
        }
    }

    return false;
}

#[allow(dead_code)]
pub fn handle_found_solution(sol: Int, period: usize, shift: usize) {
    
    println!("Found Solution: {sol}");

    log_solution_to_file(sol, period, shift);
    save_rendered_solution(sol, period, shift);
}


pub fn clear_renders() {
    let dir = Path::new("./renders");
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    let _ = fs::remove_file(path);
                }
            }
        }
    }
}

pub fn clear_output_file() {
    let _ = fs::File::create("output.txt").and_then(|mut f| f.write_all(b""));
}

