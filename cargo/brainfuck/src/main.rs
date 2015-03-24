use std::env;

use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::char;


fn main() {
    // Get the first argument to use as the filename
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => panic!("Usage: brainfuck <filename> [<tapesize>]"),
    };

    // Obtain the contents of the program file
    let bf = match read_file(&filename) {
        Ok(contents) => contents,
        Err(e) => {
            panic!(
                "Couldn't read from `{}`: {}",
                filename, e)
        }
    };

    // Initialize the tape
    let mut tape = create_tape();

    // Interpret the contents
    brainfuck(bf.as_bytes(), &mut tape);
}

fn read_file(path: &String) -> Result<String, io::Error> {
    // Open the file
    let mut file = try!(File::open(&path));

    // Read the contents of the file
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));

    Ok(contents)
}

fn create_tape() -> Box<[u32]> {
    return Box::new([0; 30000]);
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
                    0 => {
                        // Skip over all instructions until the matching `]` is
                        // reached, cancelling out matching pairs of `[` and `]`
                        // along the way.
                        let mut inner_loops = 0;
                        // Hop to the next instruction to begin looking for the
                        // matching `]`.
                        i += 1;
                        while bf[i] != 93 || inner_loops > 0 {
                            match bf[i] {
                                91 => inner_loops += 1,
                                93 => inner_loops -= 1,
                                _ => (),
                            }
                            i += 1;
                        }
                    }
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
