use std::fs::File;
use std::io::{self, Read};
use std::env;

macro_rules! dbg_println {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
        println!($($arg)*);
        }
    }
}

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut filename = "";
    let mut run_part1 = false;
    let mut run_part2 = false;

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--part1" => run_part1 = true,
            "--part2" => run_part2 = true,
            _ => filename = arg,
        }
    }

    if !run_part1 && !run_part2 {
        run_part1 = true;
        run_part2 = true;
    }

    let input = read_file(filename).expect("Unable to open input file");

    if run_part1 {
        let p1_sol = solve_p1(&input);
        println!("The password for Part 1 is: {}", p1_sol);
    }

    if run_part2 {
        let p2_sol = solve_p2(&input);
        println!("The password for Part 2 is: {}", p2_sol);
    }
}

fn solve_p1(input: &str) -> u64 {
    let mut sum = 0;
    for line in input.lines() {
        // target is largest digit + largest digit to the right of that digit, or 0
        let digits: Vec<u64> = line.bytes().map( |b| (b - b'0') as u64).collect();
        let (idx, first) = digits[..digits.len()-1].iter().enumerate().max_by_key(|(_, v)| *v).unwrap();
        dbg_println!("len: {} idx: {}", digits.len(), idx);
        let second = if idx < digits.len() {
            digits[idx..].iter().max().unwrap()
        } else {
            &0
        };
        let target = first * 10 + second;
        dbg_println!("Largest joltage of {} is {}", line, target);
        sum += target;
    }
    sum
}

fn solve_p2(input: &str) -> u64 {
    0
}
