fn main() {
    let pairs = [(2, -2), (2, 2), (3, 2), (2, 3)];

    // So I actually tried iterating through this list blindly and figured out
    // how to do it from just the error messages, neat! Looks like arrays have a
    // .iter function for returning an iterator, and that each value returned is
    // a reference. (So I had to add &s to make them work)
    for pair in pairs.iter() {
        println!("Tell me about {:?}", pair);
        match pair {
            &(x, y) if x == y => println!("These are twins"),
            &(x, y) if x + y == 0 => println!("Sum to 0"),
            &(x, _) if x%2 == 1 => println!("The first is odd"),
            _ => println!("No other matches found"),
        }
    }
}
