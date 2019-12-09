use intcode::computer::Computer;
use intcode::input::Single;
use intcode::output::PrintOutput;

fn part1() {
    let mems = Computer::get_bits("input").unwrap();

    for mem in mems {
        let mut comp = Computer::new(mem, Box::new(Single::new(1)), Box::new(PrintOutput));

        let steps = comp.run().unwrap();
        println!("Finished after {} steps", steps);
    }
}

fn main() {
    part1();
}
