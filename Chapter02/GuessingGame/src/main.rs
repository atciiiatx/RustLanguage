use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let start = 1;
    let end = 100;
    let secret_number = rand::thread_rng().gen_range(start..=end);
    println!("Guess a number between {} and {}!", start, end);

    loop {
        println!("Please input your guess.");
        let mut guess_buffer = String::new();
        io::stdin()
            .read_line(&mut guess_buffer)
            .expect("Failed to read line");
        let guess: u32 = match guess_buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations! You guessed the number!");
                break;
            }
        }
    }
    // Game ends when guessed correctly
}
