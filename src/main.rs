use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number\n");

    let mut rounds = String::new();
    println!("How many random numbers do you want to guess?");
    io::stdin()
        .read_line(&mut rounds)
        .expect("Error reading input!");

    let rounds: u8 = rounds.trim().parse().expect("Error parsing rounds");

    let mut secret_numbers = Vec::new();

    for _ in 0..rounds {
        let secret_number = rand::thread_rng().gen_range(1..=100);
        secret_numbers.push(secret_number)
    }
    // println!("The secret number is: {secret_number}");

    let mut guesses_made = 0;

    println!("Please input your guess.");
    while guesses_made < rounds {

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_numbers[guesses_made as usize]) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                guesses_made += 1;
                if guesses_made < rounds {
                    println!("Now Guess the next number, round #{}", guesses_made + 1);
                }
            }
        }
    }

    println!("Thanks for playing! The correct answers were: ");
    for item in secret_numbers {
        println!("{item}");
    }
}
