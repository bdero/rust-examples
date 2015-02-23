#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(i32, i32, i32),
}

fn main () {
    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");

    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => {
            println!("Red: {:?}, green: {:?}, and blue: {:?}!:", r, g, b);
        }
    }
}
