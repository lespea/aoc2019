pub type Bit = i32;

pub fn bit_from_bool(b: bool) -> Bit {
    if b {
        1
    } else {
        0
    }
}

pub mod error;

pub mod computer;
pub mod input;
pub mod output;
