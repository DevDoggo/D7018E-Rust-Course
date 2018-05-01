
use std::cmp::max;
use std::collections::HashMap;

pub fn print_guesses(v: &Vec<(u32, String)>) {
    let iterations = 2;
    for _i in 0..iterations { 
        println!("\nResults: Printing All\nAttempt | Guess");
        for g in v {
            println!("      {} | {}", g.0, g.1);
        }
    }
}

pub fn print_the_last_three_guesses_in_backwards_order_really_nicely(v: &Vec<(u32, String)>) {
    let mut count = 0;     
    println!("\nResults: Printing Last Three\nAttempt | Guess");
    for g in v.iter().rev() {
        println!("      {} | {}", g.0, g.1);
        count += 1;
        if count == 3 { break; }
    }
}

pub fn print_hashmap(hm: &HashMap<u32, String>) {
    println!("\nResults: Printing HashMap\nAttempt | Guess");
    for m in hm{
        println!("      {:?} | {:?}", m.0, m.1);
    }
}

pub fn print_hashmap_last_three(hm: &HashMap<u32, String>) {
    println!("\nResults: Printing Last Three in Hashmap\nAttempt | Guess");  
    let mut entries = 0;
    for _m in hm.iter() {
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

