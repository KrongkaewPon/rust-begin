use std::io;
fn main() {
    let mut a = String::new();
    println!("Enter a String");
    io::stdin().read_line(&mut a).expect("Fail");
    let a:String=a.trim().parse().expect("Fail");
    println!("{} World", a);
}