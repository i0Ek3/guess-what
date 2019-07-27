use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn guess() {
    println!("Welcome to the game: GUESS WHAT!");
    println!("You just need to input a number, are you ready?");

    let secret = rand::thread_rng().gen_range(1, 51);
    //println!("The secret number is: {}", secret);

    loop {
        println!("Please input your number:");

        let mut guess = String::new();

        io.stdin().read_line(&mut guess)
            .expect("Cannot to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Just give sth big!");
            Ordering::Greater => println!("Just small then!");
            Ordering::Equal => {
                println!("God damn, you win!");
                break;
            }
        }
    }
}

fn main() {
    guess();
}
