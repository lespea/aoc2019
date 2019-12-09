use intcode::computer::Computer;
use intcode::output::PrintOutput;
use std::collections::VecDeque;

fn part1() {
    let mem = Computer::get_bits("input").unwrap();

    if mem.len() != 1 {
        panic!("Invalid computer mem")
    }

    let mut cin = VecDeque::from(vec![1]);
    let mut cout = PrintOutput;

    Computer::new(mem.into_iter().next().unwrap(), &mut cin, &mut cout)
        .run()
        .unwrap();
}

fn main() {
    part1();
}
