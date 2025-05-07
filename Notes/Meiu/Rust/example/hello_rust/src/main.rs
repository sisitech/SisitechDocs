use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the game.");
    loop {
        let secret_number = rand::thread_rng().gen_range(1, 1001);
        println!("Secret number : {}", secret_number);
        println!("Please guess a number : ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let error_message = format!("{} is not a number.", &guess);
        // println!("{} ad",error_message.to_string());

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number !!!!!");
                continue;
            }
        };

        println!("You guessed {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Greater => {
                println!("Too big");
            }
        }
    }
}
