struct Pair<T> {
    first: T,
    second: T,
}

fn swap<T>(pair: Pair<T>) -> Pair<T> {
    let Pair { first, second } = pair;

    Pair { first: second, second: first }
}

struct Tuple2<T, U>(T, U);

fn main() {
    let pair_of_chars: Pair<char> = Pair { first: 'a', second: 'b' };

    let pair_of_ints = Pair { first: 1i32, second: 2 };

    let _tuple: Tuple2<char, i32> = Tuple2('R', 2);

    let _swapped_pair_of_chars = swap::<char>(pair_of_chars);

    let _swapped_pair_of_ints = swap(pair_of_ints);
}
