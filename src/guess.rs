use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn new_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("{}", secret_number);

    loop {
        let mut guess = String::new();

        println!("Please input your guess");

        io::stdin().read_line(&mut guess).expect("Failed to read");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!(),
        };
    
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
