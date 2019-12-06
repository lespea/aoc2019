use std::fmt::Write;

use crate::Bit;

pub trait Output {
    fn put_out(&mut self, n: Bit);
}

impl Output for Vec<Bit> {
    fn put_out(&mut self, n: Bit) {
        self.push(n);
    }
}

impl Output for dyn std::io::Write {
    fn put_out(&mut self, n: Bit) {
        writeln!(self, "{}", n).expect("Bad write");
    }
}

impl Output for dyn std::fmt::Write {
    fn put_out(&mut self, n: Bit) {
        writeln!(self, "{}", n).expect("Bad write");
    }
}

impl Output for String {
    fn put_out(&mut self, n: Bit) {
        self.write_fmt(format_args!(
            "{}{}",
            if self.is_empty() { "" } else { ", " },
            n
        ))
        .expect("Bad write");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vec_input() {
        let mut v = vec![];
        v.put_out(1);
        assert_eq!(v[0], 1);

        v.put_out(2);
        assert_eq!(v[1], 2);

        v.put_out(3);
        assert_eq!(v[2], 3);
    }

    #[test]
    fn writer() {
        let mut s = String::with_capacity(50);

        s.put_out(1);
        s.put_out(2);
        s.put_out(10);
        s.put_out(3);

        assert_eq!(s, "1, 2, 10, 3")
    }
}
