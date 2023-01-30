use std::sync::{Arc, Mutex};
use std::thread;
static mut Seat: i32 = 1;
fn seat(name: &str) {
    unsafe {
        println!("======= ");
        if Seat >= 1 {
            println!("Seat Available for {} ", name);
            println!("Booking Confirmed");
            println!("Printing Tickets for {} ", name);
            Seat -= 1;
        } else {
            println!("Seat not Available for {} ", name);
        }
    }
}

fn main() {
    // let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let name = ["Peter", "Rob"];

    for i in 0..2 {
        // let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // let mut num = counter.lock().unwrap();
            let n = name[i];
            seat(n);
            // *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
