fn main() {
    let s = String::from("Hello");
    let r = word(&s);
    println!("{}", r);
}

fn word(x: &str) -> &str {
    x
}
