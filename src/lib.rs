pub fn hello_exa() -> &'static str {
    "Hello, Exa!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_exa() {
        assert_eq!(hello_exa(), "Hello, Exa!");
    }
}
