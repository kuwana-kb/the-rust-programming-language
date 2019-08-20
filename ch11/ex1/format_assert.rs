pub fn greeting(name: &str) -> String {
    format!("Hello")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"),
        "Greeting did not contain name, value was {}", result
        )
    }
}