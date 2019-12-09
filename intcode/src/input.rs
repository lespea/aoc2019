use std::collections::VecDeque;

use crate::error::CompError::{InputErr, InputErrStr};
use crate::error::Result;
use crate::Bit;

pub trait Input {
    fn get_in(&mut self) -> Result<Bit>;
}

impl Input for VecDeque<Bit> {
    fn get_in(&mut self) -> Result<Bit> {
        self.pop_front()
            .ok_or_else(|| InputErrStr("Ran out of elements in the input vector"))
    }
}

pub struct Sinle(Bit, bool);

impl Input for Sinle {
    fn get_in(&mut self) -> Result<Bit> {
        if self.1 {
            Err(InputErrStr(""))
        } else {
            self.1 = true;
            Ok(self.0)
        }
    }
}

pub struct Interactive;

impl Input for Interactive {
    fn get_in(&mut self) -> Result<Bit> {
        dialoguer::Input::<Bit>::new()
            .with_prompt("Next input number")
            .interact()
            .map_err(|e| InputErr(Box::new(e)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vec_input() {
        let mut test = VecDeque::from(vec![1, 2, 3]);
        assert_eq!(test.get_in().unwrap(), 1);
        assert_eq!(test.get_in().unwrap(), 2);
        assert_eq!(test.get_in().unwrap(), 3);
    }

    #[test]
    #[should_panic(expected = "Ran out of elements")]
    fn vec_overflow() {
        let mut test = VecDeque::from(vec![1, 2, 3]);
        assert_eq!(test.get_in().unwrap(), 1);
        assert_eq!(test.get_in().unwrap(), 2);
        assert_eq!(test.get_in().unwrap(), 3);
        test.get_in().unwrap();
    }
}
