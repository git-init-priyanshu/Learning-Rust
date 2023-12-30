use std::io; // Standard i/o lib to take input
use colored::Colorize; // Lib to colorize text in terminal
use rand::Rng; // Lib to genrate random values
use std::cmp::Ordering; // Lib ro compare values

fn main() {
    let secret_num: u32 = rand::thread_rng().gen_range(0..100);
    println!("Guess the number game");

    loop{
        println!("Enter guess number:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line"); // Taking input (String)
        
        let guess: u32 = match guess.trim().parse() { // Converting string to u32
            Ok(num ) => num, // Checking if Input is number
            Err(_) => continue, // If not -> re-run the loop
        };

        println!("You guessed: {}",guess);

        match guess.cmp(&secret_num){ // Comparing guess and secret_number
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break // If guessed correctly then break the loop
            }
        }
    }
}
