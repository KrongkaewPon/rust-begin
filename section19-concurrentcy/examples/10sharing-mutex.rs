use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    //ex1
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6
    }

    print!("{:?}", m);

    //ex2
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result {}", *counter.lock().unwrap());
}
