use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::clone(&value);
    let b = Rc::clone(&value);
    *value.borrow_mut() += 5;
    println!("{:?}", a);
    *value.borrow_mut() += 5;
    println!("{:?}", b);
    *value.borrow_mut() += 5;
    println!("{:?}", value);
}
