// Q1. Find a alphabet is vowel or not.
use std::io;
fn main () {
    let mut input = String::new();
    println!("Enter a Char!");
    io::stdin().read_line(&mut input).expect("Fail");
    let input:char=input.trim().parse().expect("Fail");

    if input == 'a' || input == 'e' || input == 'i' || input == 'o' || input == 'u' {
        println!("vowel");
    } else {
        println!("Not!")
    }

}