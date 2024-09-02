
pub fn ch02_01() {
    println!("Guess Num Game: ch02-01 at ch02.rs file.");

}

//
// Guess Num
//
use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn ch02_02() {
    println!("Guess the number!!!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        let mut guess = String::new();

        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("joe::Failed to read line");
        // let guess: i32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(numnum) => numnum,
            Err(_) => {
                println!("{} is not number...", guess);
                continue
            }
        };
        println!("You guessed: {}.", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // loop break
            }
        }
    } // loop
}