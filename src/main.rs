fn main() {
    // In Rust, variables are immutable by default. Immutable means that after a value is bound to a name, it cannot be changed.
    let a_number = 10;
    let a_boolean = true;

    // println! macro takes a string as a first argument and one extra argument for each curly bracket pair inside the string and replaces them for the argument's display value.
    println!("The number is {}.", a_number);
    println!("The boolean is {}.", a_boolean);

    // Showing variable is immutable

    // let a_number = 10; // error: cannot assign a variable twice
    // println!("The number is {}.", a_number);
    // a_number = 15;
    // println!("and now the number is {}.", a_number);

    // if we want to be able to mutate a value, we must use the 'mut' keyword when declaring a variable to make it mutable.

    let mut a_number = 10; // mut allowing us to mutate the number
    println!("The number is {}.", a_number);
    a_number = 15;
    println!("and now the number is {}.", a_number); // we get 15

    // Shadowing allows us to declare a new variable with the same name as a previous variable, which creates a new binding. It's called shadowing because the new variable shadows the previous variable. The old variable still exists, but we can't refer to it in this scope anymore.

    let number = 5; // The first binding is created using the name "number"
    let number = number + 5; // a different binding shadows the name "number"
    let number = number * 2; // again, a new binding is created

    println!("The number is: {}", number);



}






