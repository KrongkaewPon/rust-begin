fn main() {
    let mut name = Some(String::from("Rob"));

    match name {
        Some(ref mut name) => *name = String::from("ABC"),
        None => (),
    }

    println!("{:?}", name);
}
