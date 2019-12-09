use intcode::computer::Computer;
use intcode::input::Single;
use intcode::output::PrintOutput;
use intcode::Bit;

fn run(input: Bit) {
    let mems = Computer::get_bits("input").unwrap();

    for mem in mems {
        let mut comp = Computer::new(mem, Box::new(Single::new(input)), Box::new(PrintOutput));

        let steps = comp.run().unwrap();
        println!("Finished after {} steps", steps);
    }
}

fn main() {
    // Part 1
    run(1);
}
