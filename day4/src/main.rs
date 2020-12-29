const INPUT: &str = "272091-815432";

fn main() {
    let split: Vec<usize> = INPUT
        .split("-")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let start: usize = split[0] / 100_000;
    let end: usize = split[1] / 100_000;

    let mut digits: Vec<usize> = Vec::new();
    for n in start..=end {
        digits.extend(generate(n, 5));
    }

    let mut valid1 = 0;
    let mut valid2 = 0;
    for n in digits {
        if n < split[0] {
            continue;
        }
        if n > split[1] {
            break;
        }

        let (first_valid, second_valid) = is_valid(n);
        if first_valid {
            valid1 += 1;
        }
        if first_valid && second_valid {
            valid2 += 1;
        }
    }

    println!("first {}", valid1);
    println!("second {}", valid2);
}

fn generate(digit: usize, rest: usize) -> Vec<usize> {
    if rest == 0 {
        return [digit].to_vec();
    }

    let mut digits: Vec<usize> = Vec::new();
    for d in digit % 10..=9 {
        digits.extend(generate(10 * digit + d, rest - 1));
    }

    return digits;
}

fn is_valid(pass: usize) -> (bool, bool) {
    let mut digits: Vec<usize> = pass
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    if digits.len() != 6 {
        return (false, false);
    }

    let mut repeated = false;
    let mut repeated_group = false;
    let mut same_len = 1;
    let mut same_d = 0;
    let mut lastd = digits.remove(0);

    for d in digits {
        if d < lastd {
            return (false, false);
        }

        if d == lastd {
            if same_d != d {
                if same_len == 2 {
                    repeated_group = true
                }

                same_len = 1;
            }

            repeated = true;
            same_len += 1;
            same_d = d;
        }

        lastd = d;
    }
    if same_len == 2 {
        repeated_group = true
    }

    (repeated, repeated_group)
}
