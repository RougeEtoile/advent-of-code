use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    Ok(input
        .lines()
        .map(|line| {
            let results = line
                .chars()
                .enumerate()
                .filter_map(|x: (usize, char)| match x.1.is_ascii_digit() {
                    true => Some(x.1),
                    false => starts_with(&line[x.0..]),
                })
                .collect::<Vec<char>>();
            [
                results.first().unwrap_or(&'\0').to_string(),
                results.last().unwrap_or(&'\0').to_string(),
            ]
            .join("")
            .parse::<u32>()
            .unwrap_or(0)
        })
        .sum::<u32>()
        .to_string())
}
#[tracing::instrument]
fn starts_with(word: &str) -> Option<char> {
    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    digits.iter().find_map(|&x| {
        word.starts_with(x).then_some(match x {
            "one" => '1',
            "two" => '2',
            "three" => '3',
            "four" => '4',
            "five" => '5',
            "six" => '6',
            "seven" => '7',
            "eight" => '8',
            _ => '9',
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // use rstest::rstest;
    //
    // #[rstest]
    // #[case("two1nine", 29)]
    // #[case("eightwothree", 83)]
    // #[case("abcone2threexyz", 13)]
    // #[case("xtwone3four", 24)]
    // #[case("4nineeightseven2", 42)]
    // #[case("zoneight234", 14)]
    // #[case("7pqrstsixteen", 76)]
    // #[case("fivezg8jmf6hrxnhgxxttwoneg", 51)]
    // fn line_test(#[case] line: &str, #[case] expected: u32) {
    //     assert_eq!(expected, process_line(line))
    // }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
