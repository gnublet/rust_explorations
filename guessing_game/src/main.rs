use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::Colorize;
// use std::env;
use clap::Parser;

/// Simple guessing game
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Lower bound
    #[clap(short, long, default_value_t = 0)]
    start: u32,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 100)]
    end: u32,
}
fn main() {
    let args = Args::parse();
    let lower_bound: u32 = args.start;
    let upper_bound: u32 = args.end;

    println!("====Guess the number between {} and {} (inclusive)!====\n", lower_bound, upper_bound);

    let secret_number: u32 = rand::thread_rng().gen_range(lower_bound..=upper_bound);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadowing - reusing a variable that already existed. Useful for type conversios
        // let guess: i32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", format!("Non-integer guesses are invalid.").red());
                continue;
            }
        };

        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("\n{}", format!("===You win!===").green().bold());
                break;
            }
        }
    }
    
}