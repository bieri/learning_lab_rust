extern crate rand;
use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guess the number?!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        //let guess:u32  = input.trim().parse().expect("");
        let guess:u32   =match input.trim().parse() {
                Ok(num) => num,
                Err(_)=> {
                    println!("Please input a number. Try again...");
                            continue;}
            }; 
        println!("You guessed {}",input);
        println!("The secret number is {}",secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Large"),
            Ordering::Equal => break,
        }    
    }
    
    
    
     
}
