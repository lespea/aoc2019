use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::rc::Rc;

use anyhow::{Context, Result};

fn get_pairs(s: &str) -> Result<(&str, &str)> {
    let idx = s
        .find(')')
        .with_context(|| format!("No ')' char in the orbit string `{}`", s))?;

    let (s, e) = s.split_at(idx);
    Ok((s, &e[1..]))
}

struct ToProc {
    star: Rc<String>,
    depth: usize,
}

fn calc(lines: &[String]) -> Result<usize> {
    let mut orbits = HashMap::<String, HashSet<Rc<String>>>::with_capacity(1_000);
    let mut procs = Vec::with_capacity(50);

    {
        let mut does_orbit = HashSet::with_capacity(1_000);

        for line in lines {
            let (inner, outter) = get_pairs(line)?;
            orbits
                .entry(inner.to_owned())
                .or_default()
                .insert(Rc::new(outter.to_owned()));

            does_orbit.insert(outter.to_owned());
        }

        let depth = 0;
        for k in orbits.keys() {
            if !does_orbit.contains(k) {
                let star = Rc::new(k.clone());
                procs.push(ToProc { star, depth });
            }
        }
    }

    let mut sum = 0;
    while let Some(tp) = procs.pop() {
        if let Some(parents) = orbits.get(tp.star.as_str()) {
            let depth = tp.depth + 1;
            sum += parents.len() * depth;

            for p in parents.iter() {
                procs.push(ToProc {
                    star: p.clone(),
                    depth,
                });
            }
        }
    }

    Ok(sum)
}

fn main() {
    use std::io::prelude::*;
    use std::io::BufReader;

    let l = BufReader::new(File::open("input").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    println!("{}", calc(&l).unwrap());
}

#[cfg(test)]
mod test {
    use crate::get_pairs;

    #[test]
    fn pair_split() {
        for (s, p1, p2) in &[
            ("A)B", "A", "B"),
            ("Q64)6PD", "Q64", "6PD"),
            ("A)B)", "A", "B)"),
            ("A))B", "A", ")B"),
        ] {
            let (s1, s2) = super::get_pairs(s).unwrap();
            assert_eq!(s1, *p1);
            assert_eq!(s2, *p2);
        }
    }

    #[test]
    #[should_panic]
    fn bad_pair() {
        get_pairs("ABC").unwrap();
    }

    #[test]
    fn ex1() {
        let sum = super::calc(&[
            "COM)B".to_string(),
            "B)C".to_string(),
            "C)D".to_string(),
            "D)E".to_string(),
            "E)F".to_string(),
            "B)G".to_string(),
            "G)H".to_string(),
            "D)I".to_string(),
            "E)J".to_string(),
            "J)K".to_string(),
            "K)L".to_string(),
        ])
        .unwrap();

        assert_eq!(sum, 42)
    }
}
