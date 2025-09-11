use std::io;

fn main() {
    println!("Guess the number!");

    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("The random number is: {random_number}. ")
    
    println!("Please input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
