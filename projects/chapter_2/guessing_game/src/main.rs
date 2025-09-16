use std::cmp::Ordering;
use std::io;
use std::num::ParseIntError;

fn convert_to_int(string: String) -> Result<u32, ParseIntError> {
    let string = string.trim().parse();
    return string;
}

fn main() {
    println!("Guess the number!");
    let random_number = rand::random_range(1..=100);
    //println!("The random number is: {random_number}. ");
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line. ");
        let guess: u32 = match convert_to_int(guess) {
            Ok(num) => num,
            Err(_) => {
                println!("Your input was not a number. Please guess a number.");
                continue;
            }
        };
        println!("You guessed: {guess}");
        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small! "),
            Ordering::Greater => println!("Too big! "),
            Ordering::Equal => {
                println!("You win! ");
                break;
            }
        }
    }
}
