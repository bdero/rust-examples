fn main() {
    problem1();
}

fn problem1() {
    let mut sum = 0;

    for i in 0..1000 {
        if i%3 == 0 || i%5 == 0 {
            sum += i;
        }
    }

    println!("Problem 1: {}", sum);
}
