#[cfg(test)]
mod tests {
    use string_reverse::reverse;

    #[test]
    fn test_reverse_string() {
        let input = String::from("hello");
        let expected = String::from("olleh");
        assert_eq!(reverse(input), expected);
    }

    #[test]
    fn test_empty_string() {
        let input = String::from("");
        let expected = String::from("");
        assert_eq!(reverse(input), expected);
    }
}
