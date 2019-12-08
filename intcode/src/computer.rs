use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

use crate::error::CompError::*;
use crate::error::{self, Result};
use crate::input::Input;
use crate::output::Output;
use crate::Bit;

pub struct Computer {
    pub mem: Vec<Bit>,
    idx: usize,
    input: Box<dyn Input>,
    output: Box<dyn Output>,
}

impl Computer {
    pub fn new(mem: Vec<Bit>, input: Box<dyn Input>, output: Box<dyn Output>) -> Self {
        Computer {
            mem,
            idx: 0,
            input,
            output,
        }
    }

    pub fn step(&mut self) -> Result<bool> {
        let ins = Instruction::try_from(self.mem[self.idx])?;
        self.idx += 1;

        ins.step(self)
    }

    pub fn run(&mut self) -> Result<()> {
        while self.step()? {}
        Ok(())
    }
}

#[test]
fn simple() {
    use std::collections::VecDeque;

    let mut c = Computer::new(
        vec![1002, 4, 3, 4, 33],
        Box::new(VecDeque::new()),
        Box::new(Vec::with_capacity(5)),
    );
    let steps = match c.run() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            panic!(e)
        }
    };
    println!("Finished in {} steps: {:?}", steps, c.mem);
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Mode {
    Immediate,
    Position,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Mode::*;

        f.write_str(match self {
            Immediate => "Immediate",
            Position => "Position",
        })
    }
}

impl Mode {
    fn g(b: u16, n: u16, pos: u8) -> Result<Self> {
        match (b / n) % 10 {
            1 => Ok(Mode::Immediate),
            0 => Ok(Mode::Position),
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
        }
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

    fn vals(self, comp: &mut Computer) -> Result<(Bit, Bit)> {
        Ok((
            self.m1()?.get(comp, self.cmd)?,
            self.m2()?.get(comp, self.cmd)?,
        ))
    }

    fn step(self, comp: &mut Computer) -> Result<bool> {
        use Cmd::*;
        match self.cmd {
            Add => {
                let (n1, n2) = self.vals(comp)?;
                self.m3()?.put(comp, n1 + n2, self.cmd)?;
            }

            Multiply => {
                let (n1, n2) = self.vals(comp)?;
                self.m3()?.put(comp, n1 * n2, self.cmd)?;
            }

            Input => {
                let ival = comp.input.get_in();
                self.m1()?.put(comp, ival, self.cmd)?;
            }

            Output => {
                let oval = self.m1()?.get(comp, self.cmd)?;
                comp.output.put_out(oval);
            }

            Halt => {
                comp.idx += 1;
            }
        };

        Ok(self.cmd.is_stop())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mode_m1() {
        for n in &[100, 101, 110, 1100, 1110, 1111, 11111] {
            assert_eq!(Mode::m1(*n), Mode::Immediate)
        }

        for n in &[000, 001, 010, 1000, 1010, 1011, 11011, 1002] {
            assert_eq!(Mode::m1(*n), Mode::Position)
        }
    }

    #[test]
    fn mode_m2() {
        for n in &[1000, 1010, 1100, 1100, 1110, 1111, 11111, 1002] {
            assert_eq!(Mode::m2(*n), Mode::Immediate)
        }

        for n in &[0000, 0001, 0010, 10000, 10100, 10110, 10111] {
            assert_eq!(Mode::m2(*n), Mode::Position)
        }
    }

    #[test]
    fn mode_m3() {
        for n in &[10000, 10100, 11000, 11000, 11100, 11110, 11111] {
            assert_eq!(Mode::m3(*n), Mode::Immediate)
        }

        for n in &[00000, 00001, 00010, 00000, 00100, 00110, 01111, 1002] {
            assert_eq!(Mode::m3(*n), Mode::Position)
        }
    }
}