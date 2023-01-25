#[derive(Debug)]
enum Sheet {
    Integer(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        Sheet::Integer(3),
        Sheet::Float(3.4),
        Sheet::Text(String::from("Hello")),
    ];
    println!("{:?}", row);
}
