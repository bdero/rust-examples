// It doesn't use ruby syntax for stepped ranges, so I had to look
// this one up to give it a shot.
use std::iter::range_step_inclusive;

fn main() {

    println!("What kind of person are you?");
    for age in range_step_inclusive(1, 36, 5) {
        match age {
            n @ 1 ... 12 => println!("Child of age {:?}", n),
            n @ 13 ... 19 => println!("Teen of age {:?}", n),
            n => println!("Old person of age {:?}", n),
        }
    }
}
