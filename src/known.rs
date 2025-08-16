use crate::int::*;

fn known_original() -> Vec<Int> {
    vec![
        151,
        187,
        189,
        195,
        221,
        635,
        889,
        125231,
        595703,
        610999,
        14871103,
        16537415,
        256296063,
        22503642597,
        222678959859,
        10495070598767,
        360759087837221,
        2197520782601119,
        11221488970893447375,
        142082121178470981231,
    ].into_iter().map(from_u128).collect()
}

pub fn known_reversed() -> Vec<Int> {
    let mut o: Vec<Int> = known_original()
        .into_iter()
        .flat_map(|i| [i, i.reverse_bits()].into_iter())
        .map(|i| i >> i.trailing_zeros())
        .collect();

    o.dedup();
    o
}
