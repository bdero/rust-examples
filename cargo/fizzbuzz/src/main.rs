fn main() {
    for i in 0..100 {
        println!("{}: {}", i,
                 match (i%3 == 0, i%5 == 0) {
                     (true, true) => "fizzbuzz",
                     (true, _) => "fizz",
                     (_, true) => "buzz",
                     _ => ""
                 });
    }
}
