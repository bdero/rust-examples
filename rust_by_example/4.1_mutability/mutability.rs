fn main() {
    // Immutable without the mut modifier
    let _immutable_variable = 1;
    let mut mutable_variable = 1;

    println!("Before mutation: {}", mutable_variable);

    mutable_variable += 1;

    println!("After mutation: {}", mutable_variable);

    // This doesn't compile, since immutable stuff can't be changes
    //_immutable_variable += 1;
}
