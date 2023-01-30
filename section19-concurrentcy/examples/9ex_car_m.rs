use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn body(s: mpsc::Sender<&str>) {
    for i in 1..6 {
        println!("Building Part {} ", i);
        thread::sleep(Duration::from_millis(1));
    }
    println!("Body manufactured");
    s.send("Body").unwrap();
}

fn engine(s: mpsc::Sender<&str>) {
    for i in 1..6 {
        println!("Building Engine part {} ", i);
        thread::sleep(Duration::from_millis(1));
    }
    println!("Engine Manufactured");
    s.send("Engine").unwrap();
}

fn assembly(r: mpsc::Receiver<&str>, s: mpsc::Sender<&str>) {
    if r.recv().unwrap() == "Body" && r.recv().unwrap() == "Engine" {
        println!("Thanks for producing car.");
        let mut i = 5;
        while i > 0 {
            println!("Assembling Car. Time Left {} ", i);
            thread::sleep(Duration::from_millis(1));
            i = i - 1;
        }
        println!("Car Manufactured");
        s.send("Car").unwrap();
    }
}

fn consumer(r: mpsc::Receiver<&str>) {
    if r.recv().unwrap() == "Car" {
        println!("Thanks for producing Car");
    }
}
fn main() {
    let (s, r) = mpsc::channel();
    let s1 = mpsc::Sender::clone(&s);
    let (s2, r2) = mpsc::channel();

    let handle = thread::spawn(|| {
        body(s);
        engine(s1);
        assembly(r, s2);
        consumer(r2);
    });

    handle.join().unwrap();
}
