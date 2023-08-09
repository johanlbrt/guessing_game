use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret: u8 = rand::thread_rng().gen_range(1..=100);

    let text = String::from("Let's start our guessing game!");
    println!("{}",text);

    loop {
        let text = String::from("Give me your guess: ");
        println!("{}",text);

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => {
                let text = String::from("Too less.");
                println!("{}",text);
            },
            Ordering::Equal => {
                let text = String::from("You win!");
                println!("{}",text);
                break;
            },
            Ordering::Greater => {
                let text = String::from("Too greater.");
                println!("{}",text);
            },
        }
    } 
}