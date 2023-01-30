use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let (s, r) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec!["hi", "from", "the", "thread"];

        for val in vals {
            s.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in r {
        println!("Got {}", received);
    }
}
