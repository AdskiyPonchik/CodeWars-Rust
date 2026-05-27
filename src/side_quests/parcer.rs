#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Point {
    fn from(tuple: (i32, i32)) -> Self {
        Point {
            x: tuple.0,
            y: tuple.1,
        }
    }
}
impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let mut parts = s.split(",").map(|val| val.parse::<i32>().unwrap_or(0));
        Point {
            x: parts.next().unwrap_or(0),
            y: parts.next().unwrap_or(0),
        }
    }
}

fn process_location<T: Into<Point>>(input: T) -> Point {
    input.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_tuple() {
        assert_eq!(process_location((10, 20)), Point { x: 10, y: 20 });
    }

    #[test]
    fn test_from_str() {
        assert_eq!(process_location("5,15"), Point { x: 5, y: 15 });
    }

    #[test]
    fn test_from_str_invalid() {
        assert_eq!(process_location("bad_string"), Point { x: 0, y: 0 });
    }

    #[test]
    fn test_from_point() {
        let p = Point { x: 1, y: 2 };
        assert_eq!(process_location(p), Point { x: 1, y: 2 });
    }
}
