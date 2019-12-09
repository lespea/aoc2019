use crate::input::Input;
use crate::output::Output;
use std::sync::mpsc::channel;

pub type Bit = i32;

pub fn bit_from_bool(b: bool) -> Bit {
    if b {
        1
    } else {
        0
    }
}

pub fn chan_pair(start_ins: &[Bit]) -> (Box<dyn Input>, Box<dyn Output>) {
    let (send, recv) = channel();
    for i in start_ins {
        send.send(*i).unwrap();
    }

    (Box::new(recv), Box::new(send))
}

pub mod error;

pub mod computer;
pub mod input;
pub mod output;
