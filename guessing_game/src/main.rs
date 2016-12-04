// Guessing Game
// This program will generate a random integer between one and a hundred. It will then prompt us to enter a guess. Upon entering our guess,
// it will tell us if we’re too low or too high. Once we guess correctly, it will congratulate us.

extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("Secret Number is: {}", secret_number); // Used for debugging

    loop {
	println!("Please enter your guess between 1 and 100:");
	
	let mut guess = String::new();
	
	io::stdin().read_line(&mut guess)
	    .expect("Failed to read line");
	
	let guess: u32 = match guess.trim().parse() {
	    Ok(num) => num,
	    Err(_) => continue
	};
	
	println!("Your guess is {}", guess); 
        
        match guess.cmp(&secret_number) {
	       Ordering::Less => println!("Too small"),
	       Ordering::Greater => println!("Too big!"),
	       Ordering::Equal => {
		     println!("You win!");
		     break;
	       }
	    }
    }
}
