extern crate rand;
extern crate colored;

use std::io;    //standard in-out
use std::cmp::Ordering;
use std::cmp::max;
use std::collections::HashMap;

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
    let iterations = 2;
    for i in 0..iterations { 
        println!("\nResults: Printing All\nAttempt | Guess");
        for g in v {
            println!("      {} | {}", g.0, g.1);
        }
    }
}

fn print_the_last_three_guesses_in_backwards_order_really_nicely(v: &Vec<(u32, String)>) {
    let mut count = 0;     
    println!("\nResults: Printing Last Three\nAttempt | Guess");
    for g in v.iter().rev() {
        println!("      {} | {}", g.0, g.1);
        count += 1;
        if count == 3 { break; }
    }
}

fn print_hashmap(hm: &HashMap<u32, String>) {
    println!("\nResults: Printing HashMap\nAttempt | Guess");
    for m in hm{
        println!("      {:?} | {:?}", m.0, m.1);
    }
}

fn print_hashmap_last_three(hm: &HashMap<u32, String>) {
    println!("\nResults: Printing Last Three in Hashmap\nAttempt | Guess");  
    let mut entries = 0;
    for m in hm.iter() {
        entries += 1;
    }
    //let ln: u32 = hm.len(); //Give error because hm.len() is usize, not u32
     
    for i in (max(entries-2, 1)..entries+1).rev() { 
        let mut item = hm.get(&i);
        match item {
            Some(num) => println!("      {} | {}", i, num),
            None => println!("Nothing!"),
        };
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
    
    //Hashmap
    let mut guess_hashmap = HashMap::new();

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
        
        //Vector
        guess_pair = (attempts, guess.to_string());
        guesses.push(guess_pair);

        //Hashmap
        guess_hashmap.insert(attempts, guess.to_string());
        
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

    print_guesses(&guesses);
    print_the_last_three_guesses_in_backwards_order_really_nicely(&guesses); 
    print_hashmap(&guess_hashmap);
    print_hashmap_last_three(&guess_hashmap);
}

