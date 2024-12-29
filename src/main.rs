fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
