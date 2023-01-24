#[warn(dead_code)]
struct Point<T> {
    x: T,
}
impl Point<f32> {
    fn number(&self) -> f32 {
        self.x
    }
    fn number(x) -> f32 {
        x
    }
}
impl Point<i32> {
    fn number(&self) -> i32 {
        self.x
    }
}

fn main() {
    Point::number(2);
    let n = Point { x: 2 };
    let n1 = Point { x: 2.2 };
    println!("{}", n.number());
    println!("{}", n1.number());
}
