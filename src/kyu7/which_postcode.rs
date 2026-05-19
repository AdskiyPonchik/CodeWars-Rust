fn which_postcode(s: &str) -> String {
    let s = s.trim();
    if s.len()==6 && s.chars().nth(3) == Some(' ') &&
        s.chars().take(3).all(|c| c.is_ascii_digit())
        && s.chars().skip(4).all(|c| c.is_ascii_digit()){
        return "SK".to_string();
    }

    if let Some((part1, part2)) = s.split_once(' ') {
        let part2_chars: Vec<char> = part2.chars().collect();
        let part2_valid = part2_chars.len() == 3
            && part2_chars[0].is_ascii_digit()
            && part2_chars[1].is_ascii_alphabetic()
            && part2_chars[2].is_ascii_alphabetic();

        if part2_valid {
            let chars: Vec<char> = part1.chars().collect();

            let part1_valid = match chars.as_slice() {
                [c1, c2] => c1.is_ascii_alphabetic() && c2.is_ascii_digit(),

                [c1, c2, c3] => {
                    (c1.is_ascii_alphabetic() && c2.is_ascii_alphabetic() && c3.is_ascii_digit()) ||
                        (c1.is_ascii_alphabetic() && c2.is_ascii_digit() && c3.is_ascii_digit())
                },

                [c1, c2, c3, c4] => {
                    c1.is_ascii_alphabetic() && c2.is_ascii_alphabetic() &&
                        c3.is_ascii_digit() && c4.is_ascii_digit()
                },

                _ => false,
            };

            if part1_valid {
                return "GB".to_string();
            }
        }
    }

    "Not valid".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn do_test(s: &str, exp: &str) {
        let exp = exp.to_string();
        let actual = which_postcode(s);
        assert_eq!(actual, exp, "\"{}\" should return \"{}\"", s, exp);
    }

    #[test]
    fn test_gb() {
        do_test("G4 7AH", "GB");
        do_test("G12 8NU", "GB");
        do_test("dN1 1AA", "GB");
        do_test("Se21 7AA", "GB");
        do_test("G4 7Ah  ", "GB");
    }
    #[test]
    fn test_sk() {
        do_test("040 01", "SK");
        do_test("070 08", "SK");
        do_test("  810 08", "SK");
    }

    #[test]
    fn test_not_valid() {
        do_test("G4  7AH", "Not valid");
        do_test("12 8NU", "Not valid");
        do_test("DN1 AAA", "Not valid");
        do_test("SE21 AA7", "Not valid");
        do_test("G47AH", "Not valid");
        do_test("04001", "Not valid");
    }
}