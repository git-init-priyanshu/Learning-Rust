use std::io; // Importing Standard input/output library

fn main() {
    println!("Enter your name: ");
    let mut name = String::new(); // Intialized veriable with empty string

    io::stdin().read_line(&mut name).expect("Didn't receive input value");
    /*
    * Using stdin(Standard input) from io
    * Using read_line from stdin 
    * &mut means passing mutable referance to the the read_line function
    * expect functions runs when no input is given
     */
    let greetings = "Nice to meet you";
    println!("Hello {}! {}",name.trim_end(),greetings); // trim_end removes the next line (\n) character from the string
}
