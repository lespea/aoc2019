use std::fs::File;
use std::io::BufReader;

use bstr::io::BufReadExt;

fn fuel(num: u64) -> Option<u64> {
    (num / 3).checked_sub(2)
}

fn calc_fuel(num: u64) -> u64 {
    match fuel(num) {
        Some(n) if n > 0 => num + calc_fuel(n),
        _ => num,
    }
}

fn total_fuel(num: u64) -> u64 {
    match fuel(num) {
        Some(n) => calc_fuel(n),
        _ => 0,
    }
}

fn main() {
    let f = File::open("input").expect("Missing input file");
    let reader = BufReader::new(f);

    let mut sum = 0u64;
    reader
        .for_byte_line(|line| {
            let n = String::from_utf8_lossy(line)
                .parse::<u64>()
                .expect("Invalid number found");
            sum += total_fuel(n);
            Ok(true)
        })
        .expect("Couldn't iter over input file");

    println!("{}", sum);
}

#[cfg(test)]
mod test {
    #[test]
    fn basic() {
        assert_eq!(super::total_fuel(14), 2);
    }

    #[test]
    fn small() {
        assert_eq!(super::total_fuel(1969), 966);
    }

    #[test]
    fn big() {
        assert_eq!(super::total_fuel(100756), 50346);
    }
}
