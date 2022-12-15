use std::env;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[0].clone();

    println!("Args {:?}", args);
    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut is_winning = false;

    while !is_winning {
        let mut guess = String::from("");
        println!("Please input our guess");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed");

        let guess: u32 = guess
            .trim()
            .parse()
            .expect("Failed(Wrong datatype)");

        match guess.cmp(&secret_number) {
            Ordering:: Less => println!("Too small!"),
            Ordering:: Greater => println!("Too big"),
            Ordering:: Equal => {
                is_winning = true;
                println!("You win!");
            }
        }
    }

}