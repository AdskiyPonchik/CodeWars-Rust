use std::fmt::Display;
use std::cmp::Eq;

fn into_strings<T: Display>(arr: Vec<T>) -> Vec<String>{
    arr.into_iter().map(|x| x.to_string()).collect()
}

fn print_if_equal<T:PartialEq + Display>(a:T, b:T){
    if a == b{
        println!("{}", a);
    }
}

#[cfg(test)]
mod tests {
    use super::into_strings;

    #[test]
    fn sample_tests() {
        assert_eq!(into_strings(vec![1.1, 2.2]), ["1.1", "2.2"]);
        assert_eq!(into_strings(vec![1, 2, 3]), ["1", "2", "3"]);

    }
}