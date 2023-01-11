fn main() {
    let a:i32=10;
    let a:i64=a.into();
    println!("{}",a);

    let b:i32=10;
    let b:i64=b as i64;
    println!("{}",b);
}