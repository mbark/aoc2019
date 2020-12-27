use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let trimmed = content.trim();

    let ops: Vec<usize> = trimmed
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    println!("first {}", first(&ops));
    println!("second {}", second(&ops));
}

fn first(o: &Vec<usize>) -> usize {
    let mut ops = o.clone();
    ops[1] = 12;
    ops[2] = 2;

    ops = run(ops);

    return ops[0];
}

fn second(o: &Vec<usize>) -> usize {
    let target = 19690720;

    for noun in 0..99 {
        for verb in 0..99 {
            let mut ops = o.clone();
            ops[1] = noun;
            ops[2] = verb;
            ops = run(ops);

            if target == ops[0] {
                return 100 * noun + verb;
            }
        }
    }

    return 0
}

fn run(mut ops: Vec<usize>) -> Vec<usize> {
    let mut pos = 0;

    loop {
        let op = ops[pos];
        match op {
            1 => {
                let param1 = ops[ops[pos + 1]];
                let param2 = ops[ops[pos + 2]];
                let param3 = ops[pos + 3];
                ops[param3] = param1 + param2;
            }
            2 => {
                let param1 = ops[ops[pos + 1]];
                let param2 = ops[ops[pos + 2]];
                let param3 = ops[pos + 3];
                ops[param3] = param1 * param2;
            }
            99 => break,
            _ => {
                panic!("Unexpected operation {}", op);
            }
        }

        pos += 4
    }

    ops
}
