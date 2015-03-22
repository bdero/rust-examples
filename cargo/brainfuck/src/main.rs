use std::env;

use std::str;
use std::io::prelude::*;
use std::fs::File;

use std::ops::Deref;


fn main() {
    // Use the first passed argument as the program name
    let mut args = env::args();
    args.next(); // Ignore the first argument (bin path)
    let program = args.next().unwrap();

    // Open the file
    let file = File::open(&program);
    if file.is_err() {
        panic!("Unable to open brainfuck program `{}`!", program);
    }

    // Read the contents of the file
    let mut bf = String::new();
    file.ok().unwrap().read_to_string(&mut bf);

    // Interpret the contents
    brainfuck(bf);
}

fn brainfuck(bf:String) {
    let mut tape: Vec<u16> = Vec::with_capacity(30000);
    println!("{}", bf);
}
