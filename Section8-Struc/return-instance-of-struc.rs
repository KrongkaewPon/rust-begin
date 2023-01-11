#[derive(Debug)]

struct User {
    name: String,
    age: i32
}

fn build(name:String, age:i32) -> User {
    User {
        name,
        age
    }
}

fn main() {
    let u1=build(String::from("bowwy"), 20);
    println!("{:?}", u1);
}
