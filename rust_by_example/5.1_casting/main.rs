fn main() {
    let decimal = 65.4321_f32;

    // No complicit conversion, so this errors
    //let integer: u8 = decimal;

    // Explicit conversions work
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);
}
