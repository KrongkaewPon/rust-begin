fn main() {
	let mut n=432;
	let mut c=0;
	
	while n!=0 {
		c+=1;
		n=n/10;
		println!("n={}",n);
	}
	println!("Count={}",c);

}