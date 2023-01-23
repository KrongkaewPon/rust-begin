fn main() {
    let s = String::from("Hello");
    let r = world(&s);
    println!("{}", r);
}

// fn word(s: &str) -> &str {
//     s
// }

fn word<'a>(x: &str) -> &str {
    x
}
