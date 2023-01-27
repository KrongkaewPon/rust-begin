enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use std::rc::Rc;
use List::{Cons, Nil};
fn main() {
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Counter after creating a {} ", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("Counter after creating b {} ", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("Counter after creating c {} ", Rc::strong_count(&a));
    }
    println!("Counter after dropping c {} ", Rc::strong_count(&a));
}
