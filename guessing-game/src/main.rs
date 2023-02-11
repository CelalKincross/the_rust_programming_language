use std::io;
// input output library comes from the standard library
// cargo doc --open  // will build docs of dependencies then just click on the crate you are interested in
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100); 
    println!("The secret number is {secret_number}");

    println!("Please enter your guess:");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        println!("You guessed: {guess}");

        // shadow the guess variable// shadowing allows you to use the guess variable without creating another unique variable and having two guess variables located in memory
        
        let guess: u32 = guess.trim().parse().expect("Please enter a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
}
