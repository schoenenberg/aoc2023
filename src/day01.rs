/**!
 */
pub fn solve(path: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string(path)?;
    let lines = file.lines();

    let s1: u32 = lines
        .clone()
        .map(|l| {
            let digits: Vec<char> = l.chars().filter(|c| c.is_digit(10)).collect();
            (digits.first().unwrap().to_digit(10).unwrap() * 10)
                + digits.last().unwrap().to_digit(10).unwrap()
        })
        .sum();

    let s2: u32 = lines
        .map(|l| {
            let tokens = tokenize_regex(l);
            (token_to_digit(tokens.first().unwrap()) * 10) + token_to_digit(tokens.last().unwrap())
        })
        .sum();

    Ok((s1.to_string(), s2.to_string()))
}

pub fn tokenize_regex(input: &str) -> Vec<&str> {
    static RE: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
        regex::Regex::new(
            r#"(zero|one|two|three|four|five|six|seven|eight|nine|0|1|2|3|4|5|6|7|8|9)"#,
        )
        .unwrap()
    });
    RE.find_iter(input).map(|m| m.as_str()).collect()
}

pub fn token_to_digit(s: &str) -> u32 {
    match s {
        "zero" | "0" => 0,
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => unreachable!("invalid token"),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_tokenize_1() {
        let input = "oneplustwo";
        let expected = vec!["one", "two"];
        let actual = super::tokenize_regex(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenize_2() {
        let input = "one1plus5two";
        let expected = vec!["one", "1", "5", "two"];
        let actual = super::tokenize_regex(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenize_3() {
        let input = "zoneight234";
        let expected = vec!["one", "2", "3", "4"];
        let actual = super::tokenize_regex(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenize_4() {
        let input = "xtwone3four";
        let expected = vec!["two", "3", "four"];
        let actual = super::tokenize_regex(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn evaluate_1() {
        let expected = 281;
        let actual = super::solve("testdata/day01.txt").unwrap().1;
        assert_eq!(expected.to_string(), actual);
    }
}
