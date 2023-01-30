use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn producer(s: mpsc::Sender<&str>) {
    for i in 1..6 {
        println!("Building Part {} ", i);
        thread::sleep(Duration::from_millis(1));
    }
    println!("Car is Ready for Painting");
    s.send("Man Done").unwrap();
}

fn paint(r: mpsc::Receiver<&str>, s: mpsc::Sender<&str>) {
    if r.recv().unwrap() == "Man Done" {
        for i in 1..6 {
            println!("Coloring part {} ", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
    println!("Car is Ready to Sell");
    s.send("Coloring Done").unwrap();
}

fn seller(r: mpsc::Receiver<&str>) {
    if r.recv().unwrap() == "Coloring Done" {
        println!("Thanks for producing car");
        println!("Now I will sell the car");
    }
}

fn main() {
    let (s, r) = mpsc::channel();
    let (s1, r1) = mpsc::channel();

    let handle = thread::spawn(|| {
        producer(s);
        paint(r, s1);
        seller(r1);
    });

    handle.join().unwrap();
}
