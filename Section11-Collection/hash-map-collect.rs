use std::collections::HashMap;

fn main() {
    let team = vec!["blue", "red"];
    let score = vec![10, 20];
    let scores: HashMap<_, _> = team.iter().zip(score.iter()).collect();
    println!("{:?}", scores);
}
