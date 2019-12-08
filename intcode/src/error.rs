use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::computer::{Cmd, Mode};
use crate::Bit;

#[derive(Debug)]
pub enum CompError {
    AddrOverflow(Cmd, Bit, usize, usize),
    InvalidAddress(usize, Option<Bit>, Mode, Cmd),
    InvalidInstruction(Bit),
    InvalidMode(u16, u8, u16),
    InvalidOutputMode(usize, Cmd),
}

pub type Result<T> = std::result::Result<T, CompError>;

impl Display for CompError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use CompError::*;

        match self {
            InvalidInstruction(b) => f.write_fmt(format_args!("Unknown instruction: {}", b)),

            InvalidAddress(idx, bit, mode, cmd) => f.write_fmt(format_args!(
                "Invalid address at idx {} :: bit={:?}; mode={}; cmd={}",
                idx, bit, mode, cmd
            )),

            InvalidOutputMode(idx, cmd) => f.write_fmt(format_args!(
                "The output mode cannot be immediate for the cmd {} at addr {}",
                cmd, idx,
            )),

            AddrOverflow(cmd, bit, mlen, idx) => f.write_fmt(format_args!(
                "The address for the cmd {}, {}, is greater than the mem len {} at pos {}",
                cmd, bit, mlen, idx,
            )),

            InvalidMode(bit, pos, digit) => f.write_fmt(format_args!(
                "The bit {} has an invalid digit {} for the mode at position {}",
                bit, digit, pos,
            )),
        }
    }
}

impl Error for CompError {}
