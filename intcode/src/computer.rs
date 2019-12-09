use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::path::Path;

use crate::error::CompError::*;
use crate::error::{self, Result};
use crate::input::Input;
use crate::output::Output;
use crate::{bit_from_bool, Bit};

pub struct Computer<'a, 'b> {
    pub mem: Vec<Bit>,
    idx: usize,
    rel: Bit,
    input: &'a mut dyn Input,
    output: &'b mut dyn Output,
}

impl Computer<'_, '_> {
    pub fn get_bits<P: AsRef<Path>>(p: P) -> Result<Vec<Vec<Bit>>> {
        let path = &(*p.as_ref()).to_path_buf();

        let rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(p)
            .map_err(|e| InvalidCsvError(e, path.clone()))?;

        let mut mems = Vec::with_capacity(50);

        for rec in rdr.into_records() {
            let rec = rec.map_err(|e| InvalidCsvError(e, path.clone()))?;

            let mut mem = Vec::with_capacity(rec.len());
            for r in rec.iter() {
                mem.push(
                    r.parse::<Bit>()
                        .map_err(|_| InvalidBitStr(r.to_owned(), path.clone()))?,
                );
            }

            mems.push(mem);
        }

        Ok(mems)
    }

    pub fn new<'pi, 'po>(
        mem: Vec<Bit>,
        input: &'pi mut dyn Input,
        output: &'po mut dyn Output,
    ) -> Computer<'pi, 'po> {
        Computer {
            mem,
            idx: 0,
            rel: 0,
            input,
            output,
        }
    }

    pub fn step(&mut self) -> Result<bool> {
        let idx = self.idx;
        let ins = Instruction::try_from(
            self.mem
                .get(idx)
                .copied()
                .ok_or_else(|| InvalidIndex(idx))?,
        )?;
        self.idx += 1;

        ins.step(self)
    }

    pub fn run(&mut self) -> Result<usize> {
        let mut steps = 0;
        while !self.step()? {
            steps += 1;
        }
        Ok(steps + 1)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Mode {
    Immediate,
    Position,
    Relative,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Mode::*;

        f.write_str(match self {
            Immediate => "Immediate",
            Position => "Position",
            Relative => "Relative",
        })
    }
}

impl Mode {
    fn g(b: u16, n: u16, pos: u8) -> Result<Self> {
        match (b / n) % 10 {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            2 => Ok(Mode::Relative),
            _ => Err(InvalidMode(b, pos, n)),
        }
    }

    fn m1(b: u16) -> Result<Self> {
        Mode::g(b, 100, 1)
    }

    fn m2(b: u16) -> Result<Self> {
        Mode::g(b, 1_000, 2)
    }

    fn m3(b: u16) -> Result<Self> {
        Mode::g(b, 10_000, 3)
    }

    fn get(self, comp: &mut Computer, cmd: Cmd) -> Result<Bit> {
        let idx = comp.idx;
        comp.idx += 1;

        let addr = comp
            .mem
            .get(idx)
            .copied()
            .ok_or_else(|| InvalidAddress(idx, None, self, cmd))?;

        match self {
            Mode::Immediate => Ok(addr),

            Mode::Position => usize::try_from(addr)
                .map_err(|_| InvalidAddress(idx, Some(addr), self, cmd))
                .and_then(|new_addr| {
                    comp.mem
                        .get(new_addr)
                        .copied()
                        .ok_or_else(|| InvalidAddress(idx, Some(addr), self, cmd))
                }),

            Mode::Relative => usize::try_from(comp.rel + addr)
                .map_err(|_| InvalidAddress(idx, Some(addr), self, cmd))
                .and_then(|new_addr| {
                    comp.mem
                        .get(new_addr)
                        .copied()
                        .ok_or_else(|| InvalidAddress(idx, Some(addr), self, cmd))
                }),
        }
    }

    fn addr(self, comp: &mut Computer, cmd: Cmd) -> Result<usize> {
        let idx = comp.idx;
        let addr = self.get(comp, cmd)?;
        usize::try_from(addr).map_err(|_| InvalidAddress(idx, Some(addr), self, cmd))
    }

    fn put(self, comp: &mut Computer, val: Bit, cmd: Cmd) -> Result<()> {
        let idx = comp.idx;
        comp.idx += 1;

        let abit = comp
            .mem
            .get(idx)
            .copied()
            .ok_or_else(|| InvalidAddress(idx, None, self, cmd))?;

        if self != Mode::Position {
            return Err(InvalidOutputMode(idx, cmd));
        }

        usize::try_from(abit)
            .map_err(|_| InvalidAddress(idx, Some(abit), self, cmd))
            .and_then(|a| {
                comp.mem
                    .get_mut(a)
                    .ok_or_else(|| InvalidAddress(idx, Some(abit), self, cmd))
                    .map(|o| {
                        *o = val;
                    })
            })
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Cmd {
    Add,
    Multiply,
    Input,
    Output,
    JumpTrue,
    JumpFalse,
    LessThan,
    Equals,
    AdjustRel,
    Halt,
}

impl Display for Cmd {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Cmd::*;

        f.write_str(match self {
            Add => "Add",
            Multiply => "Multiply",
            Input => "Input",
            Output => "Output",
            JumpTrue => "Jump If True",
            JumpFalse => "Jump If False",
            LessThan => "Less Than",
            Equals => "Equals",
            AdjustRel => "Adjust Relative Base",
            Halt => "Halt",
        })
    }
}

impl Cmd {
    #[inline]
    pub fn is_stop(self) -> bool {
        self == Cmd::Halt
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Instruction {
    cmd: Cmd,
    raw: u16,
}

impl TryFrom<Bit> for Instruction {
    type Error = error::CompError;

    fn try_from(b: Bit) -> Result<Self> {
        use Cmd::*;

        let raw = u16::try_from(b).map_err(|_| InvalidInstruction(b))?;

        let cmd = match raw % 100 {
            1 => Add,
            2 => Multiply,
            3 => Input,
            4 => Output,
            5 => JumpTrue,
            6 => JumpFalse,
            7 => LessThan,
            8 => Equals,
            9 => AdjustRel,
            99 => Halt,
            n => return Err(InvalidInstruction(n as Bit)),
        };

        Ok(Instruction { cmd, raw })
    }
}

impl Instruction {
    fn m1(self) -> Result<Mode> {
        Mode::m1(self.raw)
    }
    fn m2(self) -> Result<Mode> {
        Mode::m2(self.raw)
    }
    fn m3(self) -> Result<Mode> {
        Mode::m3(self.raw)
    }

    fn get_m1(self, comp: &mut Computer) -> Result<Bit> {
        self.m1()?.get(comp, self.cmd)
    }
    fn get_m2(self, comp: &mut Computer) -> Result<Bit> {
        self.m2()?.get(comp, self.cmd)
    }
    fn put_m3(self, comp: &mut Computer, v: Bit) -> Result<()> {
        self.m3()?.put(comp, v, self.cmd)
    }

    fn step(self, comp: &mut Computer) -> Result<bool> {
        use Cmd::*;
        match self.cmd {
            Add => {
                let sum = self.get_m1(comp)? + self.get_m2(comp)?;
                self.put_m3(comp, sum)?;
            }

            Multiply => {
                let prod = self.get_m1(comp)? * self.get_m2(comp)?;
                self.put_m3(comp, prod)?;
            }

            Input => {
                let ival = comp.input.get_in()?;
                self.m1()?.put(comp, ival, self.cmd)?;
            }

            Output => {
                let oval = self.get_m1(comp)?;
                comp.output.put_out(oval)?;
            }

            JumpTrue => {
                let tval = self.get_m1(comp)?;
                if tval != 0 {
                    let addr = self.m2()?.addr(comp, self.cmd)?;
                    comp.idx = addr;
                } else {
                    comp.idx += 1;
                }
            }

            JumpFalse => {
                let tval = self.get_m1(comp)?;
                if tval == 0 {
                    let addr = self.m2()?.addr(comp, self.cmd)?;
                    comp.idx = addr;
                } else {
                    comp.idx += 1;
                }
            }

            LessThan => {
                let less = self.get_m1(comp)? < self.get_m2(comp)?;
                self.put_m3(comp, bit_from_bool(less))?;
            }

            Equals => {
                let less = self.get_m1(comp)? == self.get_m2(comp)?;
                self.put_m3(comp, bit_from_bool(less))?;
            }

            AdjustRel => {
                comp.rel += self.get_m1(comp)?;
            }

            Halt => return Ok(true),
        };

        Ok(false)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn mode_m1() {
        for n in &[100, 101, 110, 1100, 1110, 1111, 11111] {
            assert_eq!(Mode::m1(*n).unwrap(), Mode::Immediate)
        }

        for n in &[0, 1, 10, 1000, 1010, 1011, 11011, 1002] {
            assert_eq!(Mode::m1(*n).unwrap(), Mode::Position)
        }
    }

    #[test]
    fn mode_m2() {
        for n in &[1000, 1010, 1100, 1100, 1110, 1111, 11111, 1002] {
            assert_eq!(Mode::m2(*n).unwrap(), Mode::Immediate)
        }

        for n in &[0, 1, 10, 10000, 10100, 10110, 10111] {
            assert_eq!(Mode::m2(*n).unwrap(), Mode::Position)
        }
    }

    #[test]
    fn mode_m3() {
        for n in &[10000, 10100, 11000, 11000, 11100, 11110, 11111] {
            assert_eq!(Mode::m3(*n).unwrap(), Mode::Immediate)
        }

        for n in &[0, 1, 10, 100, 110, 1111, 1002] {
            assert_eq!(Mode::m3(*n).unwrap(), Mode::Position)
        }
    }

    #[test]
    fn simple() {
        use std::collections::VecDeque;

        let mut cin = VecDeque::new();
        let mut cout = Vec::with_capacity(5);

        let mut c = Computer::new(vec![1002, 4, 3, 4, 33], &mut cin, &mut cout);

        let steps = match c.run() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                panic!("{}", e)
            }
        };

        assert_eq!(c.mem, vec![1002, 4, 3, 4, 99]);

        assert_eq!(steps, 2);
    }

    fn test_proc(code: &[Bit], want: &[Bit]) {
        let mut cin = VecDeque::new();
        let mut cout = Vec::with_capacity(want.len());

        Computer::new(code.to_owned(), &mut cin, &mut cout)
            .run()
            .unwrap();

        assert_eq!(cout, want);
    }

    #[test]
    fn day9_exs() {
        let same = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        test_proc(&same, &same);

        test_proc(
            &[1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0],
            &[1_219_070_632_396_864],
        );

        test_proc(&[104, 1_125_899_906_842_624, 99], &[1_125_899_906_842_624]);
    }
}
