fn main() {
	let x = fact(5);
	println!("{:?}", x)
}

fn fact(a:i32) -> (String, i32) {
	let mut fact = 1;
	for n in 1 .. a+1 {
		fact=fact*n;
	}

	(String::from("Factorial of 5:"), fact)
	// ("Factorial of 5:".to_string(), fact)
}