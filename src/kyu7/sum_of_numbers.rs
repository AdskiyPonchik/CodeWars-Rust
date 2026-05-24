fn get_sum(a: i32, b: i32) -> i32 {
    if a > b {
        return get_sum(b, a)
    }
    (a..=b).sum()
}


// See: https://doc.rust-lang.org/book/testing.html

#[cfg(test)]
mod tests {
    use super::get_sum;

    #[test]
    fn sample_tests() {
        assert_eq!(get_sum(0, 1), 1);
        assert_eq!(get_sum(1, 2), 3);
        assert_eq!(get_sum(5, -1), 14);
        assert_eq!(get_sum(505, 4), 127759);
    }
}