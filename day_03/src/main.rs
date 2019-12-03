use std::collections::HashSet;

use csv::StringRecord;

enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl Dir {
    fn from_char(c: char) -> Dir {
        use Dir::*;
        match c {
            'r' | 'R' => Right,
            'd' | 'D' => Down,
            'l' | 'L' => Left,
            'u' | 'U' => Up,
            _ => panic!("Invalid dir char: {}", c),
        }
    }
}

struct Inst {
    dir: Dir,
    steps: usize,
}

#[derive(Hash, Eq, PartialOrd, PartialEq, Ord, Debug, Default, Copy, Clone)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn right(&mut self) {
        self.y += 1;
    }

    fn down(&mut self) {
        self.x -= 1;
    }

    fn left(&mut self) {
        self.y -= 1;
    }

    fn up(&mut self) {
        self.x += 1;
    }

    fn dist(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }

    fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
    }
}

impl Inst {
    fn from_str(str: &str) -> Inst {
        let (c, num) = str.split_at(1);
        let dir = Dir::from_char(c.chars().next().expect("Must have a size of 1"));

        Inst {
            dir,
            steps: num.parse().expect("Invalid steps found"),
        }
    }

    fn mark_spots(
        &self,
        set: &mut HashSet<Pos>,
        pos: &mut Pos,
        first: bool,
        lowest: &mut Option<usize>,
    ) {
        use Dir::*;
        for _ in 0..self.steps {
            match self.dir {
                Up => pos.up(),
                Right => pos.right(),
                Down => pos.down(),
                Left => pos.left(),
            };

            if first {
                set.insert(pos.clone());
            } else if set.contains(pos) {
                let dist = pos.dist();
                let old = lowest.get_or_insert(dist);
                if dist < *old {
                    *old = dist;
                }
            }
        }
    }
}

fn main() {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("input")
        .expect("Couldn't open the input file");

    let mut s = HashSet::with_capacity(500);
    let mut lowest = None;
    let mut pos = Pos::default();

    let mut sr = StringRecord::with_capacity(1 << 4, 500);

    rdr.read_record(&mut sr).expect("Couldn't parse csv record");
    for rec in sr.iter() {
        Inst::from_str(rec).mark_spots(&mut s, &mut pos, true, &mut lowest);
    }

    pos.reset();
    rdr.read_record(&mut sr).expect("Couldn't parse csv record");
    for rec in sr.iter() {
        Inst::from_str(rec).mark_spots(&mut s, &mut pos, false, &mut lowest);
    }

    println!("{:?}", lowest);
}

#[cfg(test)]
mod test {
    //    #[test]
    //    fn basic() {
    //        assert_eq!(super::total_fuel(14), 2);
    //    }
}
