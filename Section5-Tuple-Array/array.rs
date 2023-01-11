fn main() {
	let a:[i32;5]=[3;5];
	print(a);
}

fn print(x:[i32;5]) {
	println!("len:{}", x.len());
	for n in x.iter() {
		println!("{}", n);
	}
}