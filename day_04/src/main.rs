use std::fmt::{Display, Error, Formatter};

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug, Default)]
struct Num {
    d1: u8,
    d2: u8,
    d3: u8,
    d4: u8,
    d5: u8,
    d6: u8,
}

impl Display for Num {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_fmt(format_args!(
            "Num({}{}{}{}{}{})",
            self.d1, self.d2, self.d3, self.d4, self.d5, self.d6
        ))
    }
}

impl Num {
    fn from_num(s: &str, fix: bool) -> Num {
        if s.len() != 6 {
            panic!("Invalid number")
        }

        let mut i = s.chars();

        let d1 = i.next().unwrap().to_digit(10).unwrap() as u8;
        let mut d2 = i.next().unwrap().to_digit(10).unwrap() as u8;
        let mut d3 = i.next().unwrap().to_digit(10).unwrap() as u8;
        let mut d4 = i.next().unwrap().to_digit(10).unwrap() as u8;
        let mut d5 = i.next().unwrap().to_digit(10).unwrap() as u8;
        let mut d6 = i.next().unwrap().to_digit(10).unwrap() as u8;

        if fix {
            if d2 < d1 {
                d2 = d1;
                d3 = d1;
                d4 = d1;
                d5 = d1;
                d6 = d1;
            } else if d3 < d2 {
                d3 = d2;
                d4 = d2;
                d5 = d2;
                d6 = d2;
            } else if d4 < d3 {
                d4 = d3;
                d5 = d3;
                d6 = d3;
            } else if d5 < d4 {
                d5 = d4;
                d6 = d4;
            } else if d6 < d5 {
                d6 = d5
            }
        }

        Num {
            d1,
            d2,
            d3,
            d4,
            d5,
            d6,
        }
    }

    fn up(&mut self) {
        if self.d6 < 9 {
            self.d6 += 1
        } else if self.d5 < 9 {
            self.d5 += 1;
            self.d6 = self.d5;
        } else if self.d4 < 9 {
            self.d4 += 1;
            self.d5 = self.d4;
            self.d6 = self.d4;
        } else if self.d3 < 9 {
            self.d3 += 1;
            self.d4 = self.d3;
            self.d5 = self.d3;
            self.d6 = self.d3;
        } else if self.d2 < 9 {
            self.d2 += 1;
            self.d3 = self.d2;
            self.d4 = self.d2;
            self.d5 = self.d2;
            self.d6 = self.d2;
        } else if self.d1 < 9 {
            self.d1 += 1;
            self.d2 = self.d1;
            self.d3 = self.d1;
            self.d4 = self.d1;
            self.d5 = self.d1;
            self.d6 = self.d1;
        } else {
            panic!("Out of numbers!")
        }
    }

    fn has_double(self) -> bool {
        self.d1 == self.d2
            || self.d2 == self.d3
            || self.d3 == self.d4
            || self.d4 == self.d5
            || self.d5 == self.d6
    }

    fn has_only_double(self) -> bool {
        (self.d1 == self.d2 && self.d2 != self.d3)
            || (self.d2 == self.d3 && self.d3 != self.d4 && self.d3 != self.d1)
            || (self.d3 == self.d4 && self.d4 != self.d5 && self.d4 != self.d2)
            || (self.d4 == self.d5 && self.d5 != self.d6 && self.d5 != self.d3)
            || (self.d5 == self.d6 && self.d6 != self.d4)
    }

    fn done(self, that: Num) -> bool {
        self.d1 >= that.d1
            && self.d2 >= that.d2
            && self.d3 >= that.d3
            && self.d4 >= that.d4
            && self.d5 >= that.d5
            && self.d6 >= that.d6
    }
}

fn main() {
    let mut start = Num::from_num("137683", true);
    let end = Num::from_num("596253", false);

    println!("Start = {}", start);

    let mut findings = 0usize;

    while !start.done(end) {
        if start.has_only_double() {
            //            println!("  Guest :: {}", start);
            findings += 1;
        }

        start.up();
    }

    println!("Findings: {}", findings);
}
