use std::collections::BTreeMap;

use crate::custom_error::AocError;
use itertools::Itertools;
use winnow::ascii::{newline, space1};
use winnow::{
    ascii::{digit0, digit1},
    combinator::{preceded, separated, separated_pair},
    prelude::*,
};
#[derive(Debug)]
pub struct Game {
    winners: Vec<u32>,
    owned: Vec<u32>,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut input = input;
    let card_data = parse_games(&mut input).unwrap();
    let data = card_data
        .iter()
        .map(|x: &Game| x.owned.iter().filter(|y| x.winners.contains(y)).count())
        .collect_vec();
    let store = (0..card_data.len())
        .map(|index| (index, 1))
        .collect::<BTreeMap<usize, u32>>();
    dbg!(&data);
    let result = data
        .iter()
        .enumerate()
        .fold(store, |mut acc, (index, card_score)| {
            let to_add = *acc.get(&index).unwrap();

            for i in (index + 1)..(index + 1 + *card_score as usize) {
                acc.entry(i).and_modify(|value| {
                    *value += to_add;
                });
            }
            acc
        })
        .values()
        .sum::<u32>();
    Ok(result.to_string())
}
pub fn parse_games(input: &mut &str) -> PResult<Vec<Game>> {
    separated(1.., game, newline).parse_next(input)
}
pub fn game(input: &mut &str) -> PResult<Game> {
    preceded(("Card", space1, digit1, ":", space1), rounds)
        .map(|x| Game {
            winners: x.0,
            owned: x.1,
        })
        .parse_next(input)
}

pub fn rounds(input: &mut &str) -> PResult<(Vec<u32>, Vec<u32>)> {
    separated_pair(numbers, (space1, "|", space1), numbers).parse_next(input)
}

pub fn numbers(input: &mut &str) -> PResult<Vec<u32>> {
    separated(1.., parse_digits, space1).parse_next(input)
}
fn parse_digits(input: &mut &str) -> PResult<u32> {
    digit0.try_map(|x: &str| x.parse::<u32>()).parse_next(input)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86 6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14 1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", process(input)?);
        Ok(())
    }
}
