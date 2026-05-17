pub fn repeat_str(src: &str, count: usize) -> String {
    let capacity = src.len() * count;
    let mut result = String::with_capacity(capacity);

    for _ in 0..count{
        result.push_str(src);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(repeat_str("a", 4), "aaaa");
        assert_eq!(repeat_str("hello ", 3), "hello hello hello ");
        assert_eq!(repeat_str("abc", 2), "abcabc");
        assert_eq!(repeat_str("", 0), "");
        assert_eq!(repeat_str("I", 0), "");
        assert_eq!(repeat_str("", 5), "");
        assert_eq!(repeat_str("I", 6), "IIIIII");
        assert_eq!(repeat_str("Hello", 5), "HelloHelloHelloHelloHello");
    }
}
