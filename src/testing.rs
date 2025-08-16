use crate::int::*;

fn subsequences_with_gaps(n: Int) -> Vec<Int> {
    todo!()
}


pub fn main() {
    let ns: Vec<Int> = vec![0b111000110001, 0b111100001111];
    for n in ns {
        let o = subsequences_with_gaps(n);
        println!("{:?}", o);
        for i in o {
            println!("{i:b}");
        }
    }

    assert!(false);
}