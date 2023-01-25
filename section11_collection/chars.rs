fn main() {
    let s1 = String::from("Hello");
    let n = &s1[0..2];
    println!("{}", n);

    for n in s1.chars() {
        println!("{}", n);
    }

    println!("-----",);

    for n in "Booo".chars() {
        println!("{}", n);
    }
}
