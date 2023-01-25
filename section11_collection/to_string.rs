fn main() {
    let a = 1;
    let mut s = a.to_string();
    s.push_str(" Hello");
    s.push_str(" Bow");
    println!("{}", s);
}
