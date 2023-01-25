struct S<'a> {
    name: &'a String,
}

impl<'a> S<'a> {
    fn fun(&self) -> &String {
        self.name
    }
}

fn main() {
    let s = S {
        name: &String::from("Bowwy"),
    };

    println!("{}", s.fun());
}
