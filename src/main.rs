use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main()
{
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the random number.");
    println!("Enter your guess: ");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    let guess: u32 = guess.trim().parse().expect("Cmon man, we need a number!");

    println!("Your guess was {guess}");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Guess was smaller!"),
        Ordering::Greater => println!("Guess was larger!"), 
        Ordering::Equal => {
            println!("wow! Right on the dot, you git bro");
        }
    }

    println!("The secret number was {secret_number}");


}

    
