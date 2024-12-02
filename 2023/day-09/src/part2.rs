use std::ops::Not;

use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let res = _input
        .lines()
        .map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse().expect("Should be number"))
                .collect();
            let start = std::iter::successors(Some(nums), |nums| {
                nums.iter().all(|f| *f == 0).not().then_some(
                    nums.iter()
                        .tuple_windows::<(&i32, &i32)>()
                        .map(|(left, right)| right - left)
                        .collect(),
                )
            })
            .map(|x| *x.first().unwrap())
            .collect::<Vec<i32>>();
            start.iter().rev().fold(0, |acc, e| e - acc)
        })
        .sum::<i32>();
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}



