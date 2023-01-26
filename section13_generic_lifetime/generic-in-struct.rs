#[derive(Debug)]
struct Point<T, E> {
    x: T,
    y: E,
}

fn main() {
    let integer = Point { x: 5, y: 10.8 };
    let float = Point { x: 8.0, y: 9 };
    println!("{:?}\n{:?}", integer, float);
}
