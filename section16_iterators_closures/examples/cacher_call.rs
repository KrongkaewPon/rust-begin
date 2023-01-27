use std::collections::HashMap;
use std::thread;
use std::time::Duration;
fn main() {
    call();
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        println!("========");
        println!("self.val - {:?}", self.value);
        match self.value.contains_key(&arg) {
            true => {
                println!("found");
                *self.value.get(&arg).unwrap()
            }
            false => {
                println!("not found");
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn call() {
    let mut c = Cacher::new(|a| {
        println!("Cacher!!!!");
        a
    });
    let v1 = c.value(1);
    let v2 = c.value(2);
    let v3 = c.value(1);

    assert_eq!(v2, 2);
}
