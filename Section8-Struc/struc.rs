fn main() {
    #[derive(Debug)]
    struct User {
        email: String,
        age: i32
    }

    let mut bow = User {
        email:String::from("bow@email.com"),
        age:8
    };

    println!("{:?}", bow);
    println!("{:?}", bow.age);

    bow.age = 20;
    println!("{:?}", bow.age);
    println!("{:?}", bow.email);

    let user1 = User {age:10, email:String::from("u1@email.com")};
    let user2 = User {age:20, email:String::from("u2@email.com")};
    if user1.age > user2.age {
        println!("user1 is elder");
    } else {
        println!("user2 is elder");
    }
}