fn main() {
    print!("January has ");

    println!("{} days", 31); // Just like python!

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {predicate}",
             predicate="over the lazy dog",
             subject="the quick brown fox",
             verb="jumps");

    /*
    The guide references this syntax as "special formatting". Does it support
    things like date formatting out of the box like this, or just basic type
    conversions and number systems, etc?
    */
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    println!("My name is {0}, {1} {0}", "Bond", "James");
}
