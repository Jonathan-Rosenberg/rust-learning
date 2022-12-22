use std::io; // importing the 'io' module from the standard library (package) 'std'
use rand::Rng; // 'use' == import. 'Rng' is a trait that must be in scope to use random number generators methods.
use std::cmp::Ordering;

// to read more about any crate that we import to the project, run `cargo doc --open` and choose the crate you're interested in.

fn main() {
    loop {
        println!("Guess your number!");

        // 'thread_rng()' gives us the particular random number generator we're going to use: one that is local to the current thread of execution
        // and is seeded by the OS.
        let secret_number = rand::thread_rng()
            .gen_range(1..=100); // The 'gen_range' method is defined by the 'Rng' trait. Format of input: `start..=end` inclusive in both ends

        println!("The secret number is: {secret_number}");

        println!("Please input your guess:");

        let mut guess = String::new(); // the 'mut' keyword indicates that the variable is mutable and not constant.
        // String::new(); 'new' is 'String's associated function which means that the TYPE has the 'new' funciton implemented on him, in this case the type is 'String'.
        let immutabe_guess = "a constant guess"; // every variable in rust is by default immutable.

        io::stdin() // calling 'stdin' returns a 'Stdin' instnace that has methods such as 'read_line'
            .read_line(&mut guess) // like variables, references are immutable by deafult in rust. Therefore, if we want a reference to be changed we must declare it as 'mut'.
            .expect("Failed to read line");

        // the ':' tells Rust we;ll annotate the variable's type. Parse realizes that the parsed expression should be u32.
        let guess: u32 = guess.trim().parse().expect("Please type a numer!");

        // match expression is much like a switch case expression but the terminology is a bit different:
        // 'match' is made out of `arms`. An `arm` consists of a `pattern` to match against.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You Winnnnnn!"),
        }
        
        println!("You guessed: {guess}");
    }
}
