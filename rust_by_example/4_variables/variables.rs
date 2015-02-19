fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    // This is the only construct that really requires the :? when formatting,
    // since the compiler complains about it not having a String "trait".
    // Interesting stuff.
    println!("Meet the unit value: {:?}", unit);

    // Prefixing variable names with an underscore surpresses the warning about
    // it being unused. Prefixing it with an underscore actually does change
    // its reference name.
    let _unused_variable = 3u32;
    let noisy_unused_variable = 2u32;

    println!("{}", _unused_variable);
}
