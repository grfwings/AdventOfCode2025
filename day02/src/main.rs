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

fn solve_p1(input: &str) -> i64 {
    let mut invalid_id_sum = 0;

    for range in input.split(',') {
        let (first, last) = range.split_once('-').unwrap();
        dbg_println!("{} to {}", first, last);
        let lo = first.trim().parse::<i64>().unwrap();
        let hi = last.trim().parse::<i64>().unwrap();
        for i in lo..=hi {
            let s = i.to_string();
            let (left, right) = s.split_at(s.len() / 2);
            if left == right {
                dbg_println!("Invalid ID found: {}", s);
                invalid_id_sum += i;
            }
            //dbg_println!("{}", i);
        }
    }
    invalid_id_sum
}

fn solve_p2(input: &str) -> u64 {
    let mut invalid_id_sum = 0;

    for range in input.split(',') {
        let (first, last) = range.split_once('-').unwrap();
        let lo = first.trim().parse::<u64>().unwrap();
        let hi = last.trim().parse::<u64>().unwrap();
        for n in lo..=hi {
            dbg_println!("Testing n={}",n);
            if is_invalid_math(&n) {
                invalid_id_sum += n;
            }

        }
    }

    invalid_id_sum
}

// For Part 2, must find numbers composed of sequences of repeating digits.
// Can be represented as the geometric series
// n = d * ( 10^[k*(m-1)] + 10^[k*(m-2)] + ... + 10^k + 1)
// Where d is repeating digits, k is length, and m is # repetitions >= 2
// So the factor f can be found with
// f = (10^(k*m) - 1) / (10^k - 1)
// this gives us n = f * d
// ex. 12341234 = 1234 * 10001
// so as long as n % f and n / f < 10^k, it is a repeating digit num
fn is_invalid_math(n: &u64) -> bool {
    let digit_count = n.ilog10() + 1;

    for k in 1..digit_count {
        if digit_count % k != 0 {
            continue;
        }

        let m = digit_count / k;
        let r = 10_u64.pow(k);
        let f = (r.pow(m) - 1) / (r - 1);

        if n % f == 0 && n / f < r {
            dbg_println!("Found invalid ID {}", n);
            return true;
        }
    }

    false
}

fn is_invalid_str(input: &u64) -> bool {
    let str = input.to_string();
    let doubled = format!("{}{}", str, str);
    doubled[1..].find(&str).map_or(false, |i| i < str.len() -1)
}
