#[derive(Debug)]
struct User {
	name:String,
	age:i32,
	//hobby:String,
}

fn main() {
	let u1= User {
		name:String::from("Rob"),
		age:25,
		
	};
	let u2= User {
		name:String::from("Bob"),
		..u1
	};
	println!("u1 {:?}\nu2 {:?}",u1,u2)
	
	
}