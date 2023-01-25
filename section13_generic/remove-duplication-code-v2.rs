//Fixing largest function with Trait
fn main() {
    let list = vec![23, 54, 65, 67];
    let result = largest(&list);
    println!("{}", result);
    let list = vec!['y', 't', 'u'];
    let result = largest(&list);
    println!("{}", result);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &n in list {
        if n > largest {
            largest = n;
        }
    }
    largest
}
