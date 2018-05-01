extern crate rand;
extern crate colored;

use std::io;    //standard in-out
use std::cmp::Ordering;
use rand::Rng;
use std::io::Write; //seemingly needed to use io::stdout().flush()
use colored::*;

fn get_input() -> Result<u32, String> {
    //returns io::Result, this has .expect() method
    let mut guess = String::new();
    print!("\nYour guess is...: ");
    io::stdout().flush().unwrap(); //Throws away the result, and grabs the OK if it exists.
    io::stdin().read_line(&mut guess); 

    //parse makes the string into the number type (in this case u32)
    //parse() can crash too with strange input. So .expect is here too!
    match guess.trim().parse() {
        Ok(num) => Ok(num), //Ok(num),
        Err(_) => Err(String::from("in parsing u32, ")),
    }
}

fn print_guesses(v: &Vec<(u32, String)>) {
    println!("Results:\nAttempt | Guess");
    for g in v {
        println!("      {} | {}", g.0, g.1);
    }
}

fn main() {
    println!("{}", "\n\nGuess the number or perish. :)".bold());

    //Inclusive Lower Bound, Exclusive Upper Bound (1-100 are possible)
    let secret_number = rand::thread_rng().gen_range(1, 101); 

    println!("The secret number is: {}, but don't tell a soul.", secret_number);
  
    //Vector to hold all guesses, first to last.
    let mut guesses: Vec<(u32, String)>= Vec::new();
    let mut guess_pair: (u32, String);
    let mut guess: u32;
    let mut attempts = 0;

    loop{
        match get_input() {
            Ok(num) => {
                println!("Number {}!", num);
                guess = num;
            }
            Err(error) => {
                println!("Error {}!", error);
                continue;
            }
        };
         
        attempts += 1;
        guess_pair = (attempts, guess.to_string());
        guesses.push(guess_pair);
        
        println!("\nYou guessed: {}!", guess);
        print!("That's ");

        //with guess, compare to secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                print!("{}", "too small!".red().bold());
            }
            Ordering::Greater => {
                print!("{}", "too big!".red().bold());
            }
            Ordering::Equal => {
                print!("{}", "the correct number!\n".green().bold());
                println!("And you've only guessed {} time(s)?! How?!\n", attempts);
                break;
            }
        }
        println!("\nYou've guessed {} time(s) already!", attempts.to_string().yellow())
    }
    //println!("{:?}", guesses);
    print_guesses(&guesses);
}

