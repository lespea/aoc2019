use std::collections::VecDeque;

use crate::Bit;

pub trait Input {
    fn get_in(&mut self) -> Bit;
}

impl Input for VecDeque<Bit> {
    fn get_in(&mut self) -> Bit {
        self.pop_front().expect("Ran out of input")
    }
}

pub struct Interactive;

impl Input for Interactive {
    fn get_in(&mut self) -> Bit {
        dialoguer::Input::<Bit>::new()
            .with_prompt("Next input number")
            .interact()
            .expect("Invalid number passed")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vec_input() {
        let mut test = VecDeque::from(vec![1, 2, 3]);
        assert_eq!(test.get_in(), 1);
        assert_eq!(test.get_in(), 2);
        assert_eq!(test.get_in(), 3);
    }

    #[test]
    #[should_panic(expected = "Ran out of input")]
    fn vec_overflow() {
        let mut test = VecDeque::from(vec![1, 2, 3]);
        assert_eq!(test.get_in(), 1);
        assert_eq!(test.get_in(), 2);
        assert_eq!(test.get_in(), 3);
        test.get_in();
    }
}
