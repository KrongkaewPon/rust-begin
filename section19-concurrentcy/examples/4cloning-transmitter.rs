use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let (s, r) = mpsc::channel();
    let s1 = mpsc::Sender::clone(&s);
    thread::spawn(move || {
        let vals = vec!["1hi", "2from", "3the", "4thread"];

        for val in vals {
            s1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec!["10more", "20messages", "30for", "40you"];

        for val in vals {
            s.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in r {
        println!("Got {}", received);
    }
}
