use std::sync::mpsc;
use std::thread;
fn main() {
    let (s, r) = mpsc::channel();
    // thread::spawn(|| {
    //     run(s);
    // });
    // run1(r);

    let handle = thread::spawn(|| {
        run(s);
        run1(r);
    });
    handle.join().unwrap();
}
fn run(s: mpsc::Sender<i32>) {
    s.send(2).unwrap();
    s.send(3).unwrap();
}
fn run1(r: mpsc::Receiver<i32>) {
    let rec = r.recv().unwrap();
    println!("{}", rec);
    println!("{}", r.recv().unwrap());
}
