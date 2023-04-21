//io library
use std::io;

//entry point for the program
fn main()
{
    println!("Welcome to Guess the Number Game!");

    println!("Enter your guess: ");

    //mutable guess that is bound to an empty instance of a "String"
    let mut guess = String::new();

    //calling stdin() from io, & means reference
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {guess}");

}
