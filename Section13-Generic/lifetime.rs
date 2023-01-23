fn main() {
    // v1();
    v2();
}

#[allow(dead_code)]
fn v1() {
    let s1 = "Hello";
    let s2 = "Bye";
    let result = longest(s1, s2);
    println!("{}", result);
}

fn v2() {
    let s1 = String::from("H");
    let result;
    {
        let s2 = String::from("Bye");
        result = longest(&s1, &s2);
    }
    println!("{}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
