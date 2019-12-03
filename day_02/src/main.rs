fn run(v: &mut Vec<usize>) {
    for i in (0..v.len()).step_by(4) {
        let op = v[i];
        if op == 99 {
            break;
        }

        let dst = v[i + 3];
        let src1 = v[v[i + 1]];
        let src2 = v[v[i + 2]];

        v[dst] = match op {
            1 => src1 + src2,
            2 => src1 * src2,
            _ => panic!("Bad op: {}", op),
        }
    }
}

fn main() {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("input")
        .expect("Couldn't open the input file");

    let mut nums: Vec<usize> = Vec::with_capacity(500);

    for r in rdr.records().map(|r| r.expect("Invalid row")) {
        for field in r.iter() {
            nums.push(field.parse().expect("Bad number in input"))
        }
    }

    nums.shrink_to_fit();
    println!("{}", nums.len());

    println!("{:?}", nums);
    run(&mut nums);
    println!("{:?}", nums);
}

#[cfg(test)]
mod test {
    fn test(mut v1: Vec<usize>, v2: Vec<usize>) {
        super::run(&mut v1);
        assert_eq!(v1, v2);
    }

    #[test]
    fn example() {
        test(
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        )
    }

    #[test]
    fn extras() {
        test(vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]);
        test(vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]);
        test(vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]);
        test(
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }
}
