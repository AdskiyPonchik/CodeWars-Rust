use num::BigUint;

fn s(m: u8, n: BigUint) -> BigUint {
    if m == 0 {
        return BigUint::from(1u8);
    }
    let n: BigUint= (0..m).map(|x| &n + BigUint::from(x)).product();
    let m:BigUint = (1..=m).map(BigUint::from).product();

    n/m
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::BigUint;

    #[test]
    fn small_tests() {
        let input: [(u8, u32); 10] = [(0, 53),(1, 49),(1, 101),(2, 5),(2, 99),(3, 7),(3, 32),(4, 8),(5, 17),(10, 4)];
        let expected: [u32; 10] = [1, 49, 101, 15, 4950, 84, 5984, 330, 20349, 286];
        for i in 0..10 {
            let (m, n) = input[i];
            let e = expected[i];
            assert_eq!(s(m, BigUint::from(n)), BigUint::from(e));
        }
    }

    #[test]
    fn edge_cases() {
        assert_eq!(s(0, BigUint::from(1_u8)), BigUint::from(1_u8));
        assert_eq!(s(1, BigUint::from(1_u8)), BigUint::from(1_u8));
        assert_eq!(s(0, BigUint::from(10_u8).pow(100)), BigUint::from(1_u8));
        assert_eq!(s(1, BigUint::from(10_u8).pow(100)), BigUint::from(10_u8).pow(100));
    }
}