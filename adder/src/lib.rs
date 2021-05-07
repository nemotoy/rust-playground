pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
    #[test]
    fn greeting_contains_name(){
        let result = greeting("Tom");
        assert!(
            result.contains("Tom"),
            "Greeting did not contain name, value was {}",
            result
        );
    }
}
