fn main() {
    let a_variable;

    {
        let x = 2;
        a_variable = x*x;
    }

    println!("a variable: {}", a_variable);

    let another_variable;

    // Cool, errors for use of uninitialized variable..!
    //println!("another vatiable: {}", another_variable);

    another_variable = 1;

    println!("another variable: {}", another_variable);
}
