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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Dial {
    val: u32,
    modulus: u32,
}

impl Dial {
    fn new(val: u32, modulus: u32) -> Self {
        Self {
            val: val % modulus,
            modulus,
        }
    }

    fn add(&mut self, n: u32) {
        self.val = ( self.val + n) % self.modulus;
    }

    fn sub(&mut self, n: u32) {
        let n = n % self.modulus;
        self.val = ( self.val + self.modulus - n ) % self.modulus;
    }
}

fn solve_p1(input: &str) -> u32 {
    let mut dial = Dial::new(50, 100);

    let mut zeroes: u32 = 0;

    dbg_println!("The dial starts by pointing at 50");

    for line in input.lines() { 
        let dir: &str = &line[..1];
        let num: u32 = line[1..].parse().unwrap();

        match dir {
            "R" => dial.add(num),
            "L" => dial.sub(num),
            _ => panic!("Bad value for direction"),
        };

        if dial.val == 0 {
            zeroes += 1;
        }

        dbg_println!("The dial is rotated {} to point at {}", line, dial.val);
    }

    zeroes
}

fn solve_p2(input: &str) -> u32 {
    let mut dial = Dial::new(50, 100);

    let mut clicks = 0;

    for line in input.lines() {
        let dir: &str = &line[..1];
        let num: u32 = line[1..].parse().unwrap();

        match dir {
            "R" => {
                let sav = dial.val;
                dial.add(num);
                if sav == 0 && num > 0 {
                    // Starting from 0, count complete rotations
                    clicks += num / dial.modulus;
                } else if num > dial.modulus - sav {
                    let rem = num - (dial.modulus - sav);
                    clicks += 1 + rem / dial.modulus;
                } else if dial.val == 0 && sav != 0 {
                    clicks += 1;
                }
            },
            "L" => {
                let sav = dial.val;
                dial.sub(num);

                if sav == 0 && num > 0 {
                    // Starting from 0, count complete rotations
                    clicks += num / dial.modulus;
                } else {
                    if num > sav {
                        let rem = num - sav;
                        clicks +=  1 + (rem - 1) / dial.modulus;
                    } else if dial.val == 0 && sav != 0 {
                    clicks += 1;
                    }
                }
            },
            _ => panic!("Bad value for direction"),
        };

        assert!(dial.val < dial.modulus, "Dial value {} out of range!", dial.val);

        dbg_println!("The dial is rotated {} to point at {}. Clicks: {}", line, dial.val, clicks);

    }

    clicks
}
