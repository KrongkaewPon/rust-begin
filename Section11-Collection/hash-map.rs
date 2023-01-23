use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Red", 20);
    println!("{:?}", scores);

    println!("{:?}", scores.get("Blue"));
    println!("{:?}", scores.get("Yellow"));

    println!("\n!!loop");
    for (key, value) in &scores {
        println!("{} {}", key, value);
    }

    println!("\n!!update");
    scores.insert("Red", 100);
    println!("{:?}", scores);

    //
    println!("\n!!or_insert");
    scores.entry("Blue").or_insert(200);
    scores.entry("Yellow").or_insert(20); // or_insert ถ้าไม่เคยมีค่าจะ insert
    println!("{:?}", scores);
}
