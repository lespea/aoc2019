use intcode::computer::Computer;
use intcode::input::Single;
use intcode::output::PrintOutput;
use intcode::Bit;

fn run(input: Bit) {
    let mems = Computer::get_bits("input").unwrap();

    for mem in mems {
        let mut cin = Single::new(input);
        let mut cout = PrintOutput;
        let mut comp = Computer::new(mem, &mut cin, &mut cout);

        let steps = comp.run().unwrap();
        println!("Finished after {} steps", steps);
    }
}

fn main() {
    // Part 1
    //    run(1);

    // Part 2
    run(5);
}
