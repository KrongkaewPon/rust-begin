//Adding Custom Failure Message
fn greeting(name: &str) -> String {
    format!("Hello !")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Rob");
        assert!(
            result.contains("Rob"),
            "Greeting did not contain name, value was {}",
            result
        );
    }
}
