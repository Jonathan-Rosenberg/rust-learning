use std::io; // importing the 'io' module from the standard library (package) 'std'

fn main() {
    println!("Guess your number!");

    println!("Please input your guess:");

    let mut guess = String::new(); // the 'mut' keyword indicates that the variable is mutable and not constant.
    // String::new(); 'new' is 'String's associated function which means that the TYPE has the 'new' funciton implemented on him, in this case the type is 'String'.
    let immutabe_guess = "a constant guess"; // every variable in rust is by default immutable.

    io::stdin() // calling 'stdin' returns a 'Stdin' instnace that has methods such as 'read_line'
        .read_line(&mut guess) // like variables, references are immutable by deafult in rust. Therefore, if we want a reference to be changed we must declare it as 'mut'.
        .expect("Failed to read line");
    
    println!("You guessed: {guess}");
}
