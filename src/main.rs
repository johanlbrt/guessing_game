use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("{}",secret);
    let mut text = String::from("Guess the number!");
    println!("{}",text);
    text = String::from("Please input your guess.");
    println!("{}",text);
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error to read line");

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        text = String::from("Your guess is");
        println!("{}: {}",text,guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}