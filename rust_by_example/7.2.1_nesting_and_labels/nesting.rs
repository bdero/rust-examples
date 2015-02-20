fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            break 'outer;
        }
        println!("This point won't be reached");
    }
    println!("Exit the outer loop");
}

