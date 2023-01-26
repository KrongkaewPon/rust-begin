fn main() {
    let mut s = String::from("Hello");
    s = print(s);
    println!("main: {}", s);
}

fn print(s1: String) -> String {
    println!("fn: {}", s1);
    s1
}
