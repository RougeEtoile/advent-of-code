use crate::custom_error::AocError;
use itertools::Itertools;
use nom::character::complete::{self, anychar, line_ending, space1};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{IResult, Parser};
use nom_supreme::final_parser::final_parser;
use nom_supreme::parser_ext::ParserExt;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}

impl Hand {
    fn score(&self) -> Score {
        let counts = self.cards.iter().counts();
        if counts.values().contains(&5_usize) {
            return Score::FiveKind;
        };
        if counts.values().contains(&4_usize) {
            return Score::FourKind;
        };
        if counts.values().contains(&3_usize) && counts.values().contains(&2_usize) {
            return Score::FullHouse;
        };
        if counts.values().contains(&3_usize) {
            return Score::ThreeKind;
        };
        if counts.values().filter(|f| **f == 2_usize).count() == 2_usize {
            return Score::TwoPair;
        };
        if counts.values().contains(&2_usize) {
            return Score::OnePair;
        }
        Score::HighCard
    }
}
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Score {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.score().cmp(&other.score()) {
            core::cmp::Ordering::Equal => {
                let orderings = self
                    .cards
                    .iter()
                    .zip(other.cards.iter())
                    .filter_map(|(x_card, y_card)| match x_card.cmp(y_card) {
                        core::cmp::Ordering::Equal => None,
                        ord => Some(ord),
                    })
                    .collect_vec();
                if orderings.is_empty() {
                    Ordering::Equal
                } else {
                    *orderings.first().expect("Checked len of orderings first")
                }
            }
            ord => ord,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl TryFrom<char> for Card {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::T),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            _ => Err("Invalid Char Value".to_string()),
        }
    }
}

fn parse_cards(input: &str) -> IResult<&str, [Card; 5]> {
    anychar.map_res(Card::try_from).array().parse(input)
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, (cards, bid)) = separated_pair(parse_cards, space1, complete::u32)(input)?;
    Ok((input, Hand { cards, bid }))
}
fn parse_hands(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(line_ending, parse_hand)
        .terminated(line_ending.opt())
        .parse(input)
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let hands: Result<Vec<Hand>, ()> = final_parser(parse_hands)(_input);
    let hands = hands.expect("Parser ripped");
    let res = hands
        .iter()
        .sorted()
        .enumerate()
        .map(|(index, hand)| (index + 1_usize) * hand.bid as usize)
        .sum::<usize>();

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!("6440", process(input)?);
        Ok(())
    }
}
