use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let races = _input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|f| f.parse::<usize>().ok())
                .collect_vec()
        })
        .collect_vec();
    let res = races[0]
        .iter()
        .zip(races[1].iter())
        .map(|(time, distance)| {
            //TODO: Grok Binary Search
            let mut left = 0;
            let mut right = time / 2;
            // binary search
            while left + 1 < right {
                let center = (left + right) / 2;
                // can we beat the distance with these (charge * move) times?
                if center * (time - center) > *distance {
                    right = center; // yes, can win
                } else {
                    left = center; // no, cannot win
                }
            }

            time - left - right
        })
        .product::<usize>();
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("288", process(input)?);
        Ok(())
    }
}
