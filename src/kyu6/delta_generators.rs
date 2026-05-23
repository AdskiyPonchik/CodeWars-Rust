use std::ops::Sub;

fn delta<I, T>(values: I, level: usize) -> impl Iterator<Item = T>
where
    I: IntoIterator<Item = T>,
    T: Sub<Output = T> + Copy,
{
    let mut state = vec![None; level];
    values.into_iter().filter_map(move |item| {
        (0..level)
            .try_fold(item, |val, l| {
                if let Some(prev) = state[l] {
                    state[l] = Some(val);
                    Ok(val - prev)
                } else {
                    state[l] = Some(val);
                    Err(())
                }
            })
            .ok()
    })
}

#[cfg(test)]
mod tests {
    use super::delta;

    #[test]
    fn finite_collections() {
        let input1 = vec![1, 2, 3, 4, 5, 6];
        let expected1 = vec![1, 1, 1, 1, 1];
        assert_eq!(delta(input1, 1).collect::<Vec<_>>(), expected1);

        let input2 = vec![1.5, 1.5, 1.5, 1.5, 1.5, 1.5];
        let expected2 = vec![0.0];
        assert_eq!(delta(input2, 5).collect::<Vec<_>>(), expected2);

        let input3 = vec![1, -1, 1];
        let expected3 = vec![];
        assert_eq!(delta(input3, 3).collect::<Vec<_>>(), expected3);
    }

    #[test]
    fn iterators() {
        // (infinite) iterator as input
        let input1 = std::iter::successors(Some(0), |&x| Some(x + 2));
        let expected1 = vec![2, 2, 2, 2];
        assert_eq!(delta(input1, 1).take(4).collect::<Vec<_>>(), expected1);

        // is an iterator
        let iter2 = delta(vec![2, 4, 5, 6, 8], 1);
        let expected2 = vec![2, 1, 1, 2];
        for (actual, expect) in iter2.zip(expected2) {
            assert_eq!(actual, expect);
        }

        // works as source for other iterators
        let iter3 = delta(vec![0, 4, 4, 4, 4], 1).map(|x| x + 2);
        let expected3 = Some(6);
        assert_eq!(iter3.take(1).next(), expected3);
    }

    use std::ops::Sub;

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Sub for Point {
        type Output = Self;
        fn sub(self, other: Self) -> Self::Output {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    #[test]
    fn custom_types() {
        let a = Point { x: 4, y: 10 };
        let b = Point { x: -12, y: 44 };
        let c = Point { x: 20, y: 30 };
        let input = vec![a, b, c];
        let expected = vec![Point { x: 48, y: -48 }];
        assert_eq!(delta(input, 2).collect::<Vec<_>>(), expected);
    }
}
