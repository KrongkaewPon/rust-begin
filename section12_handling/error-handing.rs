use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let output = read_with_question_mark();
    match output {
        Ok(fi) => println!("{:?}", fi),
        Err(e) => println!("{:?}", e),
    };
}

fn read() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_with_question_mark() -> Result<String, io::Error> {
    let mut f = File::open("hello2.txt")?;

    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
