fn main() {
	let a:(i32,bool,f64) =(220,true,8.5);
	//println!("{:?}",a.0);
	print(a);
}

fn print(x:(i32,bool,f64)) {

	let (a,y,z)=x;
	println!("{} {} {} ",a,y,z);
}