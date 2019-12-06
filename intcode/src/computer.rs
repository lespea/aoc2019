use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Error, Formatter};

use crate::computer::Cmd::Halt;
use crate::error::CompError::{
    AddrOverflow, InvalidAddress, InvalidInstruction, InvalidOutputMode,
};
use crate::error::{self, Result};
use crate::Bit;

#[derive(Debug)]
pub struct Computer {
    pub mem: Vec<Bit>,
    idx: usize,
}

impl Computer {
    pub fn new(mem: Vec<Bit>) -> Self {
        Computer { mem, idx: 0 }
    }

    pub fn reset(&mut self) {
        self.idx = 0;
    }

    pub fn next(&mut self) -> Result<bool> {
        let ins: Instruction = self.mem[self.idx].try_into()?;

        ins.proc(self.idx, &mut self.mem)?;

        self.idx += ins.inc;
        Ok(ins.cmd == Cmd::Halt)
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
    fn steps(&self) -> usize {
        use Cmd::*;

        match self {
            Add => 4,
            Multiply => 4,
            Input => 1,
            Output => 1,
            Halt => 1,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Mode {
    Direct,
    Indirect,
}

impl Mode {
    fn m1(b: Bit) -> Self {
        if (b / 100) & 1 > 0 {
            Mode::Direct
        } else {
            Mode::Indirect
        }
    }

    fn m2(b: Bit) -> Self {
        if (b / 1_000) & 1 > 0 {
            Mode::Direct
        } else {
            Mode::Indirect
        }
    }

    fn m3(b: Bit) -> Self {
        if (b / 10_000) & 1 > 0 {
            Mode::Direct
        } else {
            Mode::Indirect
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Modes {
    pub mode1: Mode,
    pub mode2: Mode,
    pub mode3: Mode,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Instruction {
    pub cmd: Cmd,
    pub raw: Bit,
    pub inc: usize,
    pub modes: Modes,
}

impl TryFrom<Bit> for Instruction {
    type Error = error::CompError;

    fn try_from(b: Bit) -> Result<Self> {
        use Cmd::*;

        let cmd = match b % 100 {
            1 => Add,
            2 => Multiply,
            3 => Input,
            4 => Output,
            99 => Halt,
            b => return Err(InvalidInstruction(b)),
        };

        let inc = cmd.steps();

        Ok(Instruction {
            cmd,
            raw: b,
            inc,
            modes: Modes {
                mode1: Mode::m1(b),
                mode2: Mode::m2(b),
                mode3: Mode::m3(b),
            },
        })
    }
}

impl Instruction {
    #[inline]
    fn bit(cmd: Cmd, idx: usize, mem: &mut Vec<Bit>, mode: Mode, pos: usize) -> Result<Bit> {
        let b = mem[idx + pos];

        match mode {
            Mode::Direct => Ok(b),
            Mode::Indirect => usize::try_from(b)
                .map_err(|_| InvalidAddress(cmd, b, pos))
                .map(|i| mem[i]),
        }
    }

    #[inline]
    fn triple(&self, idx: usize, mem: &mut Vec<Bit>) -> Result<(Bit, Bit, Bit)> {
        if self.modes.mode3 == Mode::Direct {
            return Err(InvalidOutputMode(self.cmd, self.raw, idx + 3));
        }

        Ok((
            Instruction::bit(self.cmd, idx, mem, self.modes.mode1, 1)?,
            Instruction::bit(self.cmd, idx, mem, self.modes.mode2, 2)?,
            Instruction::bit(self.cmd, idx, mem, self.modes.mode3, 3)?,
        ))
    }

    #[inline]
    fn triple_addr(&self, idx: usize, mem: &mut Vec<Bit>) -> Result<(Bit, Bit, usize)> {
        let (n1, n2, n3) = self.triple(idx, mem)?;

        let addr = usize::try_from(n3).map_err(|_| InvalidAddress(self.cmd, n3, 3))?;

        if addr > mem.len() {
            Err(AddrOverflow(self.cmd, n3, mem.len(), idx + 3))
        } else {
            Ok((n1, n2, addr))
        }
    }

    pub fn proc(&self, idx: usize, mem: &mut Vec<Bit>) -> Result<()> {
        use Cmd::*;

        match self.cmd {
            Add => {
                let (n1, n2, dst) = self.triple_addr(idx, mem)?;
                mem[dst] = n1 + n2;
            }

            Multiply => {
                let (n1, n2, dst) = self.triple_addr(idx, mem)?;
                mem[dst] = n1 * n2;
            }

            Halt => (),

            _ => unimplemented!(),
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mode_m1() {
        for n in &[100, 101, 110, 1100, 1110, 1111, 11111] {
            assert_eq!(Mode::m1(*n), Mode::Direct)
        }

        for n in &[000, 001, 010, 1000, 1010, 1011, 11011] {
            assert_eq!(Mode::m1(*n), Mode::Indirect)
        }
    }

    #[test]
    fn mode_m2() {
        for n in &[1000, 1010, 1100, 1100, 1110, 1111, 11111] {
            assert_eq!(Mode::m2(*n), Mode::Direct)
        }

        for n in &[0000, 0001, 0010, 10000, 10100, 10110, 10111] {
            assert_eq!(Mode::m2(*n), Mode::Indirect)
        }
    }

    #[test]
    fn mode_m3() {
        for n in &[10000, 10100, 11000, 11000, 11100, 11110, 11111] {
            assert_eq!(Mode::m3(*n), Mode::Direct)
        }

        for n in &[00000, 00001, 00010, 00000, 00100, 00110, 01111] {
            assert_eq!(Mode::m3(*n), Mode::Indirect)
        }
    }
}
