fn main() {
    let long_lived_variable = 1;

    {
        let short_lived_variable = 2;
        println!("inner short: {}", short_lived_variable);

        // This shadows the long_lived_variable outside of the block
        let long_lived_variable = 5_f32;
        println!("inner long: {}", long_lived_variable);
    }

    // Errors
    //println!("outer short: {}", short_lived_variable);

    println!("outer long: {}", long_lived_variable);
}
