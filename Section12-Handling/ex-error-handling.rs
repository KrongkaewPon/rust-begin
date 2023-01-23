fn main() {
	println!("Welcome");
	let doc=document();
	if doc==String::from("Yes") {
		println!("Thank You for Submitting Documents.");
		println!("Now Submit Fees.\n");
		let f=fees();
		if f==String::from("Yes"){
			println!("Thank You for submitting fees");
			println!("Admission Confirmed\n");
		}else {
			println!("Sorry You have not Submitted fees");
			println!("Admission Cancelled.\n");
			panic!("");
		}
	
	}else {
			println!("Sorry You have not Submitted documents");
			println!("Admission Cancelled.\n");
			panic!("");

	}
	
	
	
}
fn document () ->String {
		let doc="No";
		println!("Do you have all documents");
		if doc=="Yes" {
			println!("{}\n",doc);
			return String::from("Yes")
		}else {
			println!("{}\n",doc);
			return String::from("no")
		}
}

fn fees() ->String {
	let fees="Yes";
	println!("Please Sumbit Your Fees");
	if fees=="Ok" {
		println!("{}\n",fees);
		return String::from("Yes")
	}else{
		println!("{}\n",fees);
		return String::from("No")
	
	}
	
	

}