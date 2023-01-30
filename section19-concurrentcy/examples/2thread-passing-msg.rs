use std::sync::mpsc;
use std::thread;

fn main() {
    let (s, r) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        s.send(val).unwrap(); // owner move
                              // println!("{}", val); error
    });
    let rec = r.recv().unwrap();
    println!("Got {}", rec);
}
