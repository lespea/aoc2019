use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::DerefMut;
use std::rc::Rc;

use intcode::computer::Computer;
use intcode::error::Result;
use intcode::Bit;

#[allow(clippy::many_single_char_names)]
fn sum(mem: &[Bit], a: Bit, b: Bit, c: Bit, d: Bit, e: Bit) -> Result<Bit> {
    let (mut i2, mut o1) = intcode::chan_pair(&[b]);
    let (mut i3, mut o2) = intcode::chan_pair(&[c]);
    let (mut i4, mut o3) = intcode::chan_pair(&[d]);
    let (mut i5, mut o4) = intcode::chan_pair(&[e]);

    let mut start = VecDeque::from(vec![a, 0]);
    let end = Rc::new(RefCell::new(vec![]));
    let mut o5 = end.clone();

    let mut c1 = Computer::new(mem.to_owned(), &mut start, o1.deref_mut());
    let mut c2 = Computer::new(mem.to_owned(), i2.deref_mut(), o2.deref_mut());
    let mut c3 = Computer::new(mem.to_owned(), i3.deref_mut(), o3.deref_mut());
    let mut c4 = Computer::new(mem.to_owned(), i4.deref_mut(), o4.deref_mut());
    let mut c5 = Computer::new(mem.to_owned(), i5.deref_mut(), &mut o5);

    c1.run()?;
    c2.run()?;
    c3.run()?;
    c4.run()?;
    c5.run()?;

    let e = end.borrow()[0];
    Ok(e)
}

fn part1() {
    let mem = Computer::get_bits("input").unwrap();

    if mem.len() != 1 {
        panic!("Invalid computer mem")
    }

    let mem = &mem[0];

    let mut max = 0;

    const MAX_IN: Bit = 5;

    for a in 0..MAX_IN {
        for b in 0..MAX_IN {
            for c in 0..MAX_IN {
                for d in 0..MAX_IN {
                    for e in 0..MAX_IN {
                        let v = sum(mem, a, b, c, d, e).unwrap();
                        if v > max {
                            max = v;
                        }
                    }
                }
            }
        }
    }

    println!("{}", max);
}

#[test]
fn ex1() {
    assert_eq!(
        43210,
        sum(
            &vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
            4,
            3,
            2,
            1,
            0
        )
        .unwrap()
    );
}

#[test]
fn ex2() {
    assert_eq!(
        54321,
        sum(
            &vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ],
            0,
            1,
            2,
            3,
            4
        )
        .unwrap()
    );
}

#[test]
fn ex3() {
    assert_eq!(
        65210,
        sum(
            &vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
            ],
            1,
            0,
            4,
            3,
            2,
        )
        .unwrap()
    );
}

fn main() {
    part1()
}
