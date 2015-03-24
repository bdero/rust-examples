use std::env;

use std::io::prelude::*;
use std::fs::File;
use std::char;


fn main() {
    // Use the first passed argument as the program name
    let mut args = env::args();
    args.next(); // Ignore the first argument (bin path)

    let program = match args.next() {
        Some(path) => path,
        None => panic!("Usage: brainfuck <filename>"),
    };

    // Open the file
    let mut file = match File::open(&program) {
        Ok(file) => file,
        Err(e) =>
            panic!(
                "Couldn't open `{}`: {}",
                program, e),
    };

    // Read the contents of the file
    let mut bf = String::new();
    match file.read_to_string(&mut bf) {
        Ok(_) => (),
        Err(e) =>
            panic!(
                "Couldn't read from `{}`: {}",
                program, e),
    };

    // Initialize the tape
    let mut tape: Box<[u32]> = Box::new([0; 30000]);

    // Interpret the contents
    brainfuck(bf.as_bytes(), &mut tape);
}

fn brainfuck(bf: &[u8], tape: &mut Box<[u32]>) {
    let mut loop_stack = Vec::new();

    let mut cursor = 0;
    let mut i = 0;
    while i < bf.len() {
        match bf[i] {
            62 /* > */ => cursor += 1,
            60 /* < */ => cursor -= 1,
            43 /* + */ => tape[cursor] += 1,
            45 /* - */ => tape[cursor] -= 1,
            46 /* . */ =>
                print!("{}",
                       match char::from_u32(tape[cursor]) {
                           Some(out) => out,
                           None => ' ',
                       }),
            44 /* , */ => println!("in"),
            91 /* [ */ =>
                match tape[cursor] {
                    0 =>
                        while tape[cursor] != 93 {
                            cursor += 1;
                        },
                    _ => loop_stack.push(i),
                },
            93 /* ] */ =>
                match tape[cursor] {
                    0 => { loop_stack.pop(); },
                    _ => i = match loop_stack.last() {
                        Some(i) => *i,
                        None =>
                            panic!("Mismatching loop brackets! Char: {}", i),
                    },
                },
            _ => (),
        }
        i += 1;
    }

    println!("");
}
