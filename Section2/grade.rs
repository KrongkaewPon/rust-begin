use std::io;
fn main() {
	let mut per=String::new();
	println!("Enter your percentage");
	io::stdin().read_line(&mut per).expect("Fail");
	let per:i32=per.trim().parse().expect("Fail");
	if per>=90 {
		println!("A Grade");
	}else if per>=80  {
		println!("B Grade")
	}else if per>=70{
		println!("C Grade");
	}else if per>=60 {
		println!("D Grade");
	}else {
		println!("Fail");
	
	}
}