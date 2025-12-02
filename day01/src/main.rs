use core::panic;
use std::fs::File;
use std::io::{self, Read};
use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ModInt {
    val: u32,
    modulus: u32,
}

impl ModInt {
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

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];

    let input = read_file(filename).expect("Unable to open input file");

    let mut dial = ModInt::new(50, 100);

    let mut zeroes: u32 = 0;

    println!("The dial starts by pointing at 50");

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

        println!("The dial is rotated {} to point at {}", line, dial.val);
    }

    println!("The password is {}", zeroes);
}
