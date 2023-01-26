fn main() {
    let mut s = String::from("Hello");
    print(&mut s);
    println!("main: {}", s);
}

fn print(s1: &mut String) {
    println!("fn: {}", s1);
    s1.push_str("World")
}
