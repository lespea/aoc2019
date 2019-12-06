use crate::computer::Cmd;
use crate::Bit;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CompError {
    InvalidInstruction(Bit),
    InvalidAddress(Cmd, Bit, usize),
    InvalidOutputMode(Cmd, Bit, usize),
    AddrOverflow(Cmd, Bit, usize, usize),
}

pub type Result<T> = std::result::Result<T, CompError>;

impl Display for CompError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use CompError::*;

        match self {
            InvalidInstruction(b) => f.write_fmt(format_args!("Unknown instruction: {}", b)),

            InvalidAddress(cmd, addr, pos) => f.write_fmt(format_args!(
                "Invalid address for param {} for the  command {}: {}",
                pos, cmd, addr
            )),

            InvalidOutputMode(cmd, bit, idx) => f.write_fmt(format_args!(
                "The output mode cannot be direct for the cmd {} ({}) at addr {}",
                cmd, bit, idx,
            )),

            AddrOverflow(cmd, bit, mlen, idx) => f.write_fmt(format_args!(
                "The address for the cmd {}, {}, is greater than the mem len {} at pos {}",
                cmd, bit, mlen, idx,
            )),
        }
    }
}

impl Error for CompError {}
