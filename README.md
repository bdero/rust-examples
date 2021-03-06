Rust Examples
=============

[![Build Status](https://travis-ci.org/bdero/rust-examples.svg?branch=master)](https://travis-ci.org/bdero/rust-examples)

This is mostly for experimenting to build up my knowledge of
[Rust](http://www.rust-lang.org/).

Many of the examples will be from [Rust by Example](http://rustbyexample.com).


Building crates
---------------

There are several projects located in the `cargo` directory, including a simple
brainfuck interpreter.

To build and run a crate, simply:

1. Navigate to the directory of a crate: `cd cargo/brainfuck/`
2. Build the crate: `cargo build`
3. Run the program: `cargo run`


Running the watcher
-------------------

The `watch.py` script in the root of this repository can be used to listen for
and recompile Rust files and run them on change detection. This allows for
rapidly testing features of the language, since the Rust compiler often errors
on incorrect code, even for issues much deeper than invalid syntax.

*Note: The watcher will fail to compile some things, such as examples involving
library building.*

Here's how to use it:

1. Install the requirements: `pip install -r requirements.txt`
2. Install the Rust compiler (`rustc`): http://www.rust-lang.org/
3. Run the watcher: `python watch.py`
4. Optionally, you can make the watcher compile everything by bulk modifying
   all source files: `find rust_by_example -exec touch {} \;`
