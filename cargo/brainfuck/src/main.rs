use std::env;

use std::io::prelude::*;
use std::fs::File;


fn main() {
    // Use the first passed argument as the program name
    let mut args = env::args();
    args.next(); // Ignore the first argument (bin path)

    let program_opt = args.next();
    if program_opt.is_none() {
        panic!("Usage: brainfuck <filename>");
    }
    let program = program_opt.unwrap();

    // Open the file
    let file = File::open(&program);
    if file.is_err() {
        panic!(
            "Unable to open brainfuck program `{}`: {}!",
            program,
            file.err().unwrap().description()
        );
    }

    // Read the contents of the file
    let mut bf = String::new();
    file.ok().unwrap().read_to_string(&mut bf);

    // Initialize the tape
    let mut tape: Box<[u8]> = Box::new([0; 30000]);

    // Interpret the contents
    brainfuck(bf.as_bytes(), &mut tape);
}

fn brainfuck(bf: &[u8], tape: &mut Box<[u8]>) {
    let mut cursor = 0;
    let mut i = 0;
    while i < bf.len() {
        match (bf[i]) {
            62 /* > */ => cursor += 1,
            60 /* < */ => cursor -= 1,
            43 /* + */ => tape[cursor] += 1,
            45 /* - */ => tape[cursor] -= 1,
            46 /* . */ => print!("{}", tape[cursor]),
            44 /* , */ => println!("in"),
            91 /* [ */ => println!("begin"),
            93 /* ] */ => println!("end"),
            _ => println!("ignore"),
        }
        i += 1;
    }

    println!("");
}
