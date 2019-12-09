use std::collections::VecDeque;
use std::time::Instant;

use intcode::computer::Computer;
use intcode::output::PrintOutput;
use intcode::Bit;

fn run(ins: Bit) {
    let mem = Computer::get_bits("input").unwrap();

    if mem.len() != 1 {
        panic!("Invalid computer mem")
    }

    let mut cin = VecDeque::from(vec![ins]);
    let mut cout = PrintOutput;

    Computer::new(mem.into_iter().next().unwrap(), &mut cin, &mut cout)
        .run()
        .unwrap();
}

#[allow(unused)]
fn part1() {
    run(1)
}

fn part2() {
    run(2)
}

fn main() {
    let n = Instant::now();
    part2();
    println!("{:?}", Instant::now().duration_since(n))
}
