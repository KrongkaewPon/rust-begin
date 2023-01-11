#[allow(dead_code)]
enum Coin{
	Penny,
	Nickel,
	Dime,
	Quarter,
}


fn value_in_cents(c:Coin) -> u32 {
    match c {
        Coin::Penny => {
            println!("Penny");
            1
        },

        Coin::Nickel => {
            println!("Nickel");
            5
        },

        Coin::Dime => 10,

        Coin::Quarter => 25,
    }
}
fn main() {
    println!("{:?}", value_in_cents(Coin::Penny));
    println!("{:?}", value_in_cents(Coin::Nickel));
    println!("{:?}", value_in_cents(Coin::Dime));
}