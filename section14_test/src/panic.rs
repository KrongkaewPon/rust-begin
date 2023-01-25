struct Guess {
    value: i32,
}
impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess must be greater than 1 ,got {}", value);
        } else if value > 100 {
            panic!("Guess must be smaller than 100,got {}", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic(expected = "Guess must be smaller than 100")]
    fn check() {
        Guess::new(20);
    }
}
