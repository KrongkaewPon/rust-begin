fn main() {
    let a: i32 = 10;
    let a: i64 = a.into();
    // let a: i64 = a.into() + 10; //error
    println!("{}", a);

    let b: i32 = 10;
    let b: i64 = b as i64;
    // let b: i64 = b as i64 + 10 ok;
    println!("{}", b);
}
