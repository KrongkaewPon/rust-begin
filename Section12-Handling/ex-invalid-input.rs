use std::io;
fn main() {
	println!("Hello");
let mut no = String::new();

	   
	io::stdin().read_line(&mut no)
            .expect("Failed to read line");

	let no: u32 = match no.trim().parse() {
		Ok(num) => num,
		Err(_) =>1,
	};
	println!("{}",no);
}
