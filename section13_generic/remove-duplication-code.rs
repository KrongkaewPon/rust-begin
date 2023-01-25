//Removing Duplication by Extracting a Function
fn main() {
    let list = vec![23, 54, 65, 67];
    let result = largest(&list);
    println!("{}", result);

    let list = vec!['y', 't', 'u'];
    let result = largest_char(&list);
    println!("{}", result);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &n in list {
        if n > largest {
            largest = n;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &n in list {
        if n > largest {
            largest = n;
        }
    }
    largest
}
