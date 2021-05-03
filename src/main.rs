fn main() {
    // In Rust, variables are immutable by default. Immutable means that after a value is bound to a name, it cannot be changed.
    let a_number = 10;
    let a_boolean = true;

    // println! macro takes a string as a first argument and one extra argument for each curly bracket pair inside the string and replaces them for the argument's display value.
    println!("The number is {}.", a_number);
    println!("The boolean is {}.", a_boolean);

    // let a_number = 10; // error: cannot assign a variable twice
    // println!("The number is {}.", a_number);
    // a_number = 15;
    // println!("and now the number is {}.", a_number);

    // if we want to be able to mutate a value, we must use the 'mut' keyword when declaring a variable to make it mutable.

    let mut a_number = 10; // mut allowing us to mutate the number
    println!("The number is {}.", a_number);
    a_number = 15;
    println!("and now the number is {}.", a_number);

}






