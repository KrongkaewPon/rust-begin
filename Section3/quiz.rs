use std::io;
fn main() {
	let mut ans=String::new();
	println!("Which is the longest river in world");
	for n in 1 .. 4 {
		ans.clear();
		io::stdin().read_line(&mut ans).expect("Fail");
		ans=ans.trim().to_string();
		if ans=="Nile" {
			println!("You guessed Correctly");
			break;
		}else {
			if n!=3 {
				println!("Try Again {}", n);
			}else {
				println!("You are Failed {}", n);
			}
		}
		
	}

}