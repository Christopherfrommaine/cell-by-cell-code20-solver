use crate::int::{Int, to_u8};

pub fn bits(n: Int, len: usize) -> Vec<u8> {
    (0..len)
        .map(|i| to_u8((n >> i) & 1))
        .rev()
        .collect()
}

pub fn code20(n: Int) -> Int {
    let a = n << 2;
    let b = n << 1;
    let c = n;
    let d = n >> 1;
    let e = n >> 2;
    (a ^ b ^ c ^ d ^ e) ^ (a | b | c | d | e)
}

#[allow(dead_code)]
pub fn run(n: Int, steps: usize) -> Int {
    let mut state = n;
    for _ in 0..steps {
        state = code20(state);
    }
    state
}

/// Runs for n steps, including the 0th, so output.len() == steps + 1
#[allow(dead_code)]
pub fn run_all(n: Int, steps: usize) -> Vec<Int> {
    let mut states = Vec::with_capacity(steps + 1);
    states.push(n);
    for _ in 0..steps {
        states.push(code20(states[states.len() - 1]));
    }
    states
}