use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Welcome to the number guessing game");
    println!("You must guess the number between 1 and 100\n");

    let mut guesses = 0;
    loop {
        guesses+=1;
        println!("Please guess a number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the guessed number");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please only enter integer values\n");
                continue
            },
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess was too low"),
            Ordering::Greater => println!("Your guess was too high"),
            Ordering::Equal => {
                println!("Congradulations!  You guessed correctly!");
                println!("It took you {guesses} tries");
                break;
            }
        }
    }
}
