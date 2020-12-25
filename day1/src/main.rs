use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let modules: Vec<usize> = io::BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|s| s.unwrap().parse::<usize>().unwrap())
        .collect();

    let mass = first(&modules);
    println!("first {}", mass);
    println!("second {}", second(&modules));
}

fn first(modules: &Vec<usize>) -> usize {
    return modules.iter().map(|n| fuel(n)).sum();
}

fn second(modules: &Vec<usize>) -> usize {
    return modules.iter().map(|n| with_fuel(n)).sum();
}

fn with_fuel(module: &usize) -> usize {
    if *module <= 6 {
        return 0;
    }

    let fuel = &fuel(&module);
    fuel + with_fuel(&fuel)
}

fn fuel(module: &usize) -> usize {
    module / 3 - 2
}
