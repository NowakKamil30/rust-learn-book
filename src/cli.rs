use std::env;
use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[0].clone();

    println!("Args {:?}", args);
    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut is_winning = false;

    while !is_winning {
        let mut guess = String::new();
        println!("Please input our guess");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed");

        let guess: u32 = match guess
            .trim()
            .parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering:: Less => println!("{}", "Too small!".red()),
            Ordering:: Greater => println!("{}", "Too big!".red()),
            Ordering:: Equal => {
                is_winning = true;
                println!("{}", "You win!".yellow());
            }
        }
    }

}