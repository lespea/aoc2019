use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use crate::Bit;
use crate::computer::{Cmd, Mode};

#[derive(Debug)]
pub enum CompError {
    AddrOverflow(Cmd, Bit, usize, usize),
    InvalidAddress(usize, Option<Bit>, Mode, Cmd),
    InvalidIndex(usize),
    InvalidInstruction(Bit),
    InvalidMode(u16, u8, u16),
    InvalidOutputMode(usize, Cmd),
    InputErr(Box<dyn std::error::Error>),
    InputErrStr(&'static str),
    OutputErr(Box<dyn std::error::Error>),
    OutputErrStr(&'static str),
    InvalidCsvError(csv::Error, PathBuf),
    InvalidBitStr(String, PathBuf),
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

            InvalidIndex(idx) => f.write_fmt(format_args!("The current index {} is invalid", idx)),

            InputErr(e) => f.write_fmt(format_args!("There was an issue getting the input: {}", e)),
            OutputErr(e) => {
                f.write_fmt(format_args!("There was an issue getting the output: {}", e))
            }

            InputErrStr(e) => {
                f.write_fmt(format_args!("There was an issue getting the input: {}", e))
            }
            OutputErrStr(e) => {
                f.write_fmt(format_args!("There was an issue getting the output: {}", e))
            }

            InvalidCsvError(e, path) => f.write_fmt(format_args!(
                "There was an issue getting a csv entry from {}: {}",
                path.display(),
                e
            )),
            InvalidBitStr(s, path) => f.write_fmt(format_args!(
                "Couldn't convert the bit str {} into a bit in the file {}",
                s,
                path.display()
            )),
        }
    }
}

impl Error for CompError {}
