// Q2. Create Basic Calculator which support +,-,/,* .
use std::io;
fn main() {
    let mut o = String::new();
    let a = 10;
    let b = 20;
    println!("Choose operation to be performed");
    println!(" 1. + \n 2. - \n 3. * \n 4, /");

    io::stdin().read_line(&mut o).expect("Fail");
    // let o:char=o.trim().parse().expect("Fail");
    o=o.trim().to_string();

    if o =="+" {
        println!("{}", a+b)
    } else if o =="-" {
        println!("{}", a-b)
    } else if o =="*" {
        println!("{}", a*b)
    } else if o =="/" {
        println!("{}", a/b)
    } else  {
        println!("Wrong Choose")
    }



}