extern crate rand;
extern crate colored;

use std::io;    //standard in-out
use std::cmp::Ordering;
use rand::Rng;
use std::io::Write; //seemingly needed to use io::stdout().flush()
use colored::*;

fn main() {
    println!("{}", "\n\nGuess the number or perish. :)".bold());

    //Inclusive Lower Bound, Exclusive Upper Bound (1-100 are possible)
    let secret_number = rand::thread_rng().gen_range(1, 101); 

    println!("The secret number is: {}, but don't tell a soul.", secret_number);
    
    let mut attempts = 0;

    loop{
        let mut guess = String::new();
        print!("\nYour guess is...: ");
        io::stdout().flush();

        //returns io::Result, this has .expect() method
        io::stdin().read_line(&mut guess)  
            .expect("Failed to read line"); 

        //trim removes any whitespace at start and endand \n!!!!!
        //parse makes the string into the number type (in this case u32)
        //parse() can crash too with strange input. So .expect is here too!
        let guess: u32 = match guess.trim().parse() {
            //.expect("Please type a number!"); //crash + error message
            Ok(num) => num,
            Err(_) => {
                println!("\nYou input a non-number! \n\
                        Fool, your input has been discarded!");
                continue
            }
        };

        attempts += 1;
        
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
}

