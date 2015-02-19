#![feature(core)]

fn main() {
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);
    println!("1 - 2 = {}", 1u32 - 2); // Try changing to lu32; it loops around
    // Not specifying a type for the number seems to default to u32
    println!("1 - 2 = {}", 1 - 2);

    // :04b formats to 4 binary digits
    println!("true AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("true OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("true XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("One million is written as {}", 1_000_000u32);
}
