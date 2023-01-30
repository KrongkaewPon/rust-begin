use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn borrow(s: mpsc::Sender<&str>) {
    println!("I have no money");
    let mut i = 5;
    while i > 0 {
        println!("I will return Money in {} days", i);
        thread::sleep(Duration::from_millis(1));
        i -= 1;
    }

    println!("I have money\nYou can take it.");
    s.send("Money").unwrap();
}
fn friend(r: mpsc::Receiver<&str>) {
    if r.recv().unwrap() == "Money" {
        println!("Thanks for returning money");
    }
}

fn main() {
    let (s, r) = mpsc::channel();

    let handle = thread::spawn(|| {
        borrow(s);
        friend(r);
    });

    handle.join().unwrap();
}
