use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("Hello.txt") {
            Ok(file) => file,
            Err(e) => {
                panic!("Not able to create file {:?}", e);
            }
        },
        Err(error) => {
            panic!("Unable to open File - {}", error);
        }
    };

    println!("{:?}", f);
}
