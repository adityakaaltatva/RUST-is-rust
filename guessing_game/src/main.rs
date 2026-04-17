//ask for user input, process that input, and check that the input is in the expected form

use std::io;

fn main(){
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess=String::new();
    //The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String. This new function creates a new, empty string.    // create a mutable variable to store the user input, and initialize it to an empty string
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");
    println!("The secret is owned by the user, so it can be guessed correctly. The secret number is: {guess}");
   /*
   the line .read_line(&mut guess) calls the read_line method on the standard input handle to get input from the user. We’re also passing &mut guess as the argument to read_line to tell it what string to store the user input in. The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents),
    so we therefore pass that string as an argument. The string argument needs to be mutable so the method can change the string’s content. */
}
//