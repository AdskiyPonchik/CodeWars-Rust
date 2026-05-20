pub fn split_string(s: &str) -> Vec<String> {
    s.chars().collect::<Vec<char>>().chunks(2)
        .map(|chunk| match chunk {
            [a,b] => format!("{}{}", a, b),
            [a] => format!("{}_", a),
            _ => unreachable!(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(split_string("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(split_string("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(split_string(""), [] as [&str; 0]);
    }
}
