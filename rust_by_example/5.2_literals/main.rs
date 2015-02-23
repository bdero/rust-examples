fn main() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // The type depends on how these are used
    let i = 1;
    let f = 1.0;

    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    // Sizes change when these are uncommented
    let _constraint_i = x + i;
    let _constraint_f = z + f;
}
