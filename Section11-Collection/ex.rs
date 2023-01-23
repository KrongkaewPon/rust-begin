use std::collections::HashMap;
use std::io;

fn main() {
    let mut n = String::new();
    let mut name = String::new();
    let mut no = String::new();
    let mut v_name: Vec<String> = Vec::new();
    let mut v_no: Vec<String> = Vec::new();
    let mut c = 1;
    let mut choice = String::new();

    println!("How many contacts you want to store");
    io::stdin().read_line(&mut n).expect("Fail");
    let n: u32 = n.trim().parse().expect("Fail");

    while c <= n {
        name.clear();
        no.clear();
        println!("Enter name than no.");

        io::stdin().read_line(&mut name).expect("Fail");
        let name: String = name.trim().parse().expect("Fail");

        io::stdin().read_line(&mut no).expect("Fail");
        let no: String = no.trim().parse().expect("Fail");

        v_name.push(name);
        v_no.push(no);
        c += 1;
    }

    println!("{:?} {:?}", v_name, v_no);

    let contact: HashMap<&String, &String> = v_name.iter().zip(v_no.iter()).collect();
    println!("{:?}", contact);

    println!("Which Name to Search?");
    io::stdin().read_line(&mut choice).expect("Fail");
    let choice: String = choice.trim().parse().expect("Fail");

    println!("{:?}", contact.get(choice));
    // search(contact, &choice);
}

// fn search(contact: HashMap<&String, &String>, choice: &String) {
//     for (k, v) in contact {
//         if k == choice {
//             println!("Contract {} {}", k, v);
//         }
//     }
// }
