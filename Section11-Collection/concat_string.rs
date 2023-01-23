fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = s1 + &s2;
    println!("{}", s3); // print s1 ไม่ได้ value borrowed here after move

    let s4 = String::from("Hello");
    let s5 = String::from("World");
    let s6 = format!("{}{}", s4, s5);
    println!("{}", s6);
}
