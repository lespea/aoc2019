use crossbeam::scope;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::time::Instant;

use bus::Bus;
use intcode::computer::Computer;
use intcode::error::Result;
use intcode::Bit;

#[allow(clippy::many_single_char_names)]
fn sum1(mem: &[Bit], a: Bit, b: Bit, c: Bit, d: Bit, e: Bit) -> Result<Bit> {
    let (mut i2, mut o1) = intcode::chan_pair(&[b]);
    let (mut i3, mut o2) = intcode::chan_pair(&[c]);
    let (mut i4, mut o3) = intcode::chan_pair(&[d]);
    let (mut i5, mut o4) = intcode::chan_pair(&[e]);

    let mut start = VecDeque::from(vec![a, 0]);
    let end = Rc::new(RefCell::new(vec![]));
    let mut o5 = end.clone();

    let mut c1 = Computer::new(mem.to_owned(), &mut start, &mut o1);
    let mut c2 = Computer::new(mem.to_owned(), &mut i2, &mut o2);
    let mut c3 = Computer::new(mem.to_owned(), &mut i3, &mut o3);
    let mut c4 = Computer::new(mem.to_owned(), &mut i4, &mut o4);
    let mut c5 = Computer::new(mem.to_owned(), &mut i5, &mut o5);

    c1.run()?;
    c2.run()?;
    c3.run()?;
    c4.run()?;
    c5.run()?;

    let v = end.borrow();
    if v.len() != 1 {
        panic!("Invalid output len");
    }
    Ok(v[0])
}

#[allow(unused)]
fn part1() {
    let mem = Computer::get_bits("input").unwrap();

    if mem.len() != 1 {
        panic!("Invalid computer mem")
    }

    let mem = &mem[0];

    let mut max = 0;

    const MIN_IN: Bit = 0;
    const MAX_IN: Bit = 5;

    for a in MIN_IN..MAX_IN {
        for b in MIN_IN..MAX_IN {
            if b == a {
                continue;
            }

            for c in MIN_IN..MAX_IN {
                if c == a || c == b {
                    continue;
                }

                for d in MIN_IN..MAX_IN {
                    if d == a || d == b || d == c {
                        continue;
                    }

                    for e in MIN_IN..MAX_IN {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }

                        let v = sum1(mem, a, b, c, d, e).unwrap();
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

#[allow(clippy::many_single_char_names)]
fn sum2(mem: &[Bit], a: Bit, b: Bit, c: Bit, d: Bit, e: Bit) -> Result<Bit> {
    let mut bus = Bus::new(5);
    let mut start = bus.add_rx();
    bus.broadcast(a);
    bus.broadcast(0);

    let mut outs = bus.add_rx();

    let (mut i2, mut o1) = intcode::chan_pair(&[b]);
    let (mut i3, mut o2) = intcode::chan_pair(&[c]);
    let (mut i4, mut o3) = intcode::chan_pair(&[d]);
    let (mut i5, mut o4) = intcode::chan_pair(&[e]);

    scope(|s| {
        s.spawn(move |_| {
            Computer::new(mem.to_owned(), &mut start, &mut o1)
                .run()
                .unwrap()
        });
        s.spawn(move |_| {
            Computer::new(mem.to_owned(), &mut i2, &mut o2)
                .run()
                .unwrap()
        });
        s.spawn(move |_| {
            Computer::new(mem.to_owned(), &mut i3, &mut o3)
                .run()
                .unwrap()
        });
        s.spawn(move |_| {
            Computer::new(mem.to_owned(), &mut i4, &mut o4)
                .run()
                .unwrap()
        });
        s.spawn(move |_| {
            Computer::new(mem.to_owned(), &mut i5, &mut bus)
                .run()
                .unwrap()
        });

        Ok(outs.iter().last().unwrap())
    })
    .unwrap()
}

fn part2() {
    let mem = Computer::get_bits("input").unwrap();

    if mem.len() != 1 {
        panic!("Invalid computer mem")
    }

    let mem = &mem[0];

    let mut max = 0;

    const MIN_IN: Bit = 5;
    const MAX_IN: Bit = 10;

    for a in MIN_IN..MAX_IN {
        for b in MIN_IN..MAX_IN {
            if b == a {
                continue;
            }

            for c in MIN_IN..MAX_IN {
                if c == a || c == b {
                    continue;
                }

                for d in MIN_IN..MAX_IN {
                    if d == a || d == b || d == c {
                        continue;
                    }

                    for e in MIN_IN..MAX_IN {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }

                        let v = sum2(mem, a, b, c, d, e).unwrap();
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
        43_210,
        sum1(
            &[3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
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
        54_321,
        sum1(
            &[
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
        65_210,
        sum1(
            &[
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

#[test]
fn ex4() {
    assert_eq!(
        139_629_729,
        sum2(
            &[
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5
            ],
            9,
            8,
            7,
            6,
            5,
        )
        .unwrap()
    )
}

#[test]
fn ex5() {
    assert_eq!(
        18_216,
        sum2(
            &[
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
            ],
            9,
            7,
            8,
            5,
            6,
        )
        .unwrap()
    )
}

fn main() {
    let n = Instant::now();
    part2();
    println!("{:?}", Instant::now().duration_since(n))
}
