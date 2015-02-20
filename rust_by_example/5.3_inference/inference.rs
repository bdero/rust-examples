fn main() {
    let elem = 5u8;

    let mut vec = Vec::new();

    // The compiler infers that vec is a Vec<u8>
    vec.push(elem);

    println!("{:?}", vec);
}
