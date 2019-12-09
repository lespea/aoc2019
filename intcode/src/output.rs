use std::cell::RefCell;
use std::fmt::Write;
use std::rc::Rc;
use std::sync::mpsc::Sender;

use crate::error::CompError::OutputErr;
use crate::error::Result;
use crate::Bit;

pub trait Output {
    fn put_out(&mut self, n: Bit) -> Result<()>;
}

impl Output for Vec<Bit> {
    fn put_out(&mut self, n: Bit) -> Result<()> {
        self.push(n);
        Ok(())
    }
}

impl Output for Rc<RefCell<Vec<Bit>>> {
    fn put_out(&mut self, n: Bit) -> Result<()> {
        let mut b = self.as_ref().borrow_mut();
        b.push(n);
        Ok(())
    }
}

impl Output for dyn std::io::Write {
    fn put_out(&mut self, n: Bit) -> Result<()> {
        writeln!(self, "{}", n).map_err(|e| OutputErr(Box::new(e)))
    }
}

impl Output for dyn std::fmt::Write {
    fn put_out(&mut self, n: Bit) -> Result<()> {
        writeln!(self, "{}", n).map_err(|e| OutputErr(Box::new(e)))
    }
}

impl Output for String {
    fn put_out(&mut self, n: Bit) -> Result<()> {
        self.write_fmt(format_args!(
            "{}{}",
            if self.is_empty() { "" } else { ", " },
            n
        ))
        .map_err(|e| OutputErr(Box::new(e)))
    }
}

pub struct PrintOutput;

impl Output for PrintOutput {
    fn put_out(&mut self, n: Bit) -> Result<()> {
        println!("{}", n);
        Ok(())
    }
}

impl Output for Sender<Bit> {
    fn put_out(&mut self, n: Bit) -> Result<()> {
        self.send(n).map_err(|e| OutputErr(Box::new(e)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vec_input() {
        let mut v = vec![];
        v.put_out(1).unwrap();
        assert_eq!(v[0], 1);

        v.put_out(2).unwrap();
        assert_eq!(v[1], 2);

        v.put_out(3).unwrap();
        assert_eq!(v[2], 3);
    }

    #[test]
    fn writer() {
        let mut s = String::with_capacity(50);

        s.put_out(1).unwrap();
        s.put_out(2).unwrap();
        s.put_out(10).unwrap();
        s.put_out(3).unwrap();

        assert_eq!(s, "1, 2, 10, 3")
    }
}
