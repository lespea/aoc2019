use crossbeam::unbounded;
use crossbeam::Receiver;
use crossbeam::Sender;

pub type Bit = i64;

pub fn bit_from_bool(b: bool) -> Bit {
    if b {
        1
    } else {
        0
    }
}

pub fn chan_pair(start_ins: &[Bit]) -> (Receiver<Bit>, Sender<Bit>) {
    let (send, recv) = unbounded();
    for i in start_ins {
        send.send(*i).unwrap();
    }

    (recv, send)
}

pub mod error;

pub mod computer;
pub mod input;
pub mod output;
