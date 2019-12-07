use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Error, Formatter};

use crate::computer::Cmd::Halt;
use crate::error::CompError::{
    AddrOverflow, InvalidAddress, InvalidInstruction, InvalidOutputMode,
};
use crate::error::{self, Result};
use crate::input::Input;
use crate::output::Output;
use crate::Bit;
use std::collections::VecDeque;

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

    pub fn reset(&mut self) {
        self.idx = 0;
    }

    pub fn next(&mut self) -> Result<bool> {
        let ins: Instruction = self.mem[self.idx].try_into()?;

        ins.proc(
            self.idx,
            &mut self.mem,
            self.input.as_mut(),
            self.output.as_mut(),
        )?;

        self.idx += ins.inc;
        Ok(ins.cmd == Cmd::Halt)
    }

    pub fn run(&mut self) -> Result<usize> {
        let mut steps = 0;
        while !self.next()? {
            steps += 1;
        }
        Ok(steps)
    }
}

#[test]
fn simple() {
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
            Add => 3,
            Multiply => 3,
            Input => 1,
            Output => 1,
            Halt => 1,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum Mode {
    Immediate,
    Position,
}

impl Mode {
    fn m1(b: Bit) -> Self {
        if (b / 100) & 1 > 0 {
            Mode::Immediate
        } else {
            Mode::Position
        }
    }

    fn m2(b: Bit) -> Self {
        if (b / 1_000) & 1 > 0 {
            Mode::Immediate
        } else {
            Mode::Position
        }
    }

    fn m3(b: Bit) -> Self {
        if (b / 10_000) & 1 > 0 {
            Mode::Immediate
        } else {
            Mode::Position
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
    fn bit(cmd: Cmd, idx: usize, mem: &Vec<Bit>, mode: Mode, pos: usize) -> Result<Bit> {
        let b = mem[idx + pos];
        println!("Idx: {}; mode: {:?}, pos: {} => raw: {}", idx, mode, pos, b);

        match mode {
            Mode::Immediate => Ok(b),
            Mode::Position => usize::try_from(b)
                .map_err(|_| InvalidAddress(cmd, b, pos))
                .map(|i| mem[i]),
        }
    }

    #[inline]
    fn bit_addr(cmd: Cmd, idx: usize, mem: &Vec<Bit>, mode: Mode, pos: usize) -> Result<usize> {
        let b = Instruction::bit(cmd, idx, mem, mode, pos)?;
        Instruction::addr(b, cmd, pos, mem.len(), idx)
    }

    #[inline]
    fn triple(&self, idx: usize, mem: &Vec<Bit>) -> Result<(Bit, Bit, Bit)> {
        if self.modes.mode3 == Mode::Immediate {
            return Err(InvalidOutputMode(self.cmd, self.raw, idx + 3));
        }

        Ok((
            Instruction::bit(self.cmd, idx, mem, self.modes.mode1, 1)?,
            Instruction::bit(self.cmd, idx, mem, self.modes.mode2, 2)?,
            Instruction::bit(self.cmd, idx, mem, self.modes.mode3, 3)?,
        ))
    }

    #[inline]
    fn addr(bit: Bit, cmd: Cmd, pos: usize, m_len: usize, idx: usize) -> Result<usize> {
        let addr = usize::try_from(bit).map_err(|_| InvalidAddress(cmd, bit, pos))?;

        if addr > m_len {
            Err(AddrOverflow(cmd, bit, m_len, idx))
        } else {
            Ok(addr)
        }
    }

    #[inline]
    fn triple_addr(&self, idx: usize, mem: &Vec<Bit>) -> Result<(Bit, Bit, usize)> {
        let (n1, n2, n3) = self.triple(idx, mem)?;
        println!("{:?} :: N1: {}; N2: {}; N3: {}", self.modes, n1, n2, n3);
        Ok((
            n1,
            n2,
            Instruction::addr(n3, self.cmd, 3, mem.len(), idx + 3)?,
        ))
    }

    pub fn proc(
        &self,
        idx: usize,
        mem: &mut Vec<Bit>,
        input: &mut dyn Input,
        output: &mut dyn Output,
    ) -> Result<()> {
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

            Input => {
                let addr = Instruction::bit_addr(self.cmd, idx, mem, self.modes.mode1, 1)?;
                mem[addr] = input.get_in();
            }

            Output => {
                output.put_out(Instruction::bit(self.cmd, idx, mem, self.modes.mode1, 1)?);
            }

            Halt => (),
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
