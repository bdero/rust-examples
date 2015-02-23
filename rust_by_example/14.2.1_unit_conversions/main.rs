use std::ops::Add;
use std::marker::PhantomData;

struct Inch;

struct Mm;

struct Length<Unit, T>(T, PhantomData<Unit>);

impl<Unit, T:Add<T, Output=T> + Copy> Add<Length<Unit, T>> for Length<Unit, T> {
    type Output = Length<Unit, T>;

    fn add(self, r: Length<Unit, T>) -> Length<Unit, T> {
        let Length(ref left, _) = self;
        let Length(ref right, _) = r;

        Length(*left + *right, PhantomData)
    }
}

fn main() {
    let one_foot: Length<Inch, f32> = Length(12.0, PhantomData);
    let one_meter: Length<Mm, f32> = Length(1000.0, PhantomData);

    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("1 + 1 foot = {:?}", two_feet);
    println!("1 + 1 meter = {:?}", two_meters);
}
