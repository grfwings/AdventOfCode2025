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
    val: i32,
    size: i32,
}

impl Dial {
    fn new(val: i32, size: i32) -> Self {
        Self {
            val,
            size,
        }
    }

    fn rotate(&mut self, n: i32) {
        self.val = ( self.val + n ).rem_euclid(self.size);
    }
}

fn solve_p1(input: &str) -> u32 {
    let mut dial = Dial::new(50, 100);
    let mut zeroes: u32 = 0;

    dbg_println!("The dial starts by pointing at 50");

    for line in input.lines() { 
        let dir: &str = &line[..1];
        let num: i32 = line[1..].parse().unwrap();

        match dir {
            "R" => dial.rotate(num),
            "L" => dial.rotate(-num),
            _ => panic!("Bad value for direction"),
        };

        if dial.val == 0 {
            zeroes += 1;
        }

        dbg_println!("The dial is rotated {} to point at {}", line, dial.val);
    }

    zeroes
}

fn solve_p2(input: &str) -> i32 {

    let mut pos: i32 = 50;
    let size: i32 = 100;
    let mut clicks: i32 = 0;

    for line in input.lines() {
        let dir: &str = &line[..1];
        let num: i32 = line[1..].parse().unwrap();
        let old_pos = pos;
        let rotation = if dir == "R" { pos + num } else {  pos - num };

        clicks += rotation.abs() / size;

        if rotation * old_pos <= 0 {
            clicks += 1;
        }
        if old_pos == 0 {
            clicks -= 1;
        }

        pos = rotation % size;

        dbg_println!("The dial is rotated {} to point at {}. Clicks: {}", line, pos, clicks);

    }

    clicks
}
