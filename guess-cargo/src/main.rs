extern crate rand;
use rand::Rng;
use std::io;
fn main() {
    println!("Guess a number");
    let secret =rand::thread_rng().gen_range(0..10);
    loop {
        println!("input your guess");
        let mut guess=String::new();
        io::stdin().read_line(&mut guess).expect("Fail");
        let guess:i32=guess.trim().parse().expect("Fail");
        if guess==secret {
            println!("Guessed Correctly!");
            break;
        } else {
            if guess>secret {
                println!("higher!");
            } else {
                println!("lower!");
            }
        }
    }
}
