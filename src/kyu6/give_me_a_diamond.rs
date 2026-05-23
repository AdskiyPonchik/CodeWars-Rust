fn print(n: i32) -> Option<String> {
    if n < 0 || n % 2 == 0 {
        return None;
    }
    let mut s = String::new();
    let mid = n / 2;
    for i in 0..n {
        let dist = (i - mid).abs();
        s.push_str(&format!("{}{}\n", " ".repeat(dist as usize), "*".repeat((n - 2 * dist) as usize)));
    }
    Some(s)
}

#[test]
fn basic_test() {
    assert_eq!(print(3), Some(" *\n***\n *\n".to_string()) );
    assert_eq!(print(5), Some("  *\n ***\n*****\n ***\n  *\n".to_string()) );
    assert_eq!(print(-3),None);
    assert_eq!(print(2),None);
    assert_eq!(print(0),None);
    assert_eq!(print(1), Some("*\n".to_string()) );
}
