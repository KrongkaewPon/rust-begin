#[allow(dead_code)]
enum Coin{
	Penny,
	Nickel,
	Dime,
	Quarter(i32),
}
#[allow(dead_code)]
#[derive(Debug)]
enum UsState{
	Alaska,
	Arizona,
	//etc
}
fn value_in_cents(c:Coin)->u32{
	match c {
		Coin::Penny=>1,
		Coin::Nickel=>5,
		Coin::Dime=>10,
		Coin::Quarter(state)=>{
			println!("{}",state);
			25
		},
	}

}

fn main(){
	
	println!("{}",value_in_cents(Coin::Quarter(2)));
	
	
	
	
}