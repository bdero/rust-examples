fn main() {
    let x = 5u32;

    // Very neat; everything is an expression, including blocks, and
    // the last expression in a block is assigned to the l-value.
    let y = {
        let x_squared = x*x;
        let x_cube = x_squared*x;

        x_cube + x_squared + x
    };

    let z = {
        2*x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
