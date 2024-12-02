use crate::custom_error::AocError;
use itertools::Itertools;
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
            Score::FiveKind
        } else if counts.values().contains(&4_usize) {
            Score::FourKind
        } else if counts.values().contains(&3_usize) && counts.values().contains(&2_usize) {
            Score::FullHouse
        } else if counts.values().contains(&3_usize) {
            Score::ThreeKind
        } else if counts.values().filter(|f| **f == 2_usize).count() == 2_usize {
            Score::TwoPair
        } else if counts.values().contains(&2_usize) {
            Score::OnePair
        } else {
            Score::HighCard
        }
    }
    fn joker_score(&self) -> Score {
        match (
            self.score(),
            self.cards.iter().filter(|card| **card == Card::J).count(),
        ) {
            (Score::FiveKind, _) => Score::FiveKind,
            (Score::FourKind, 4) => Score::FiveKind,
            (Score::FourKind, 1) => Score::FiveKind,
            (Score::FourKind, _) => Score::FourKind,
            (Score::FullHouse, 3) => Score::FiveKind,
            (Score::FullHouse, 2) => Score::FiveKind,
            (Score::FullHouse, _) => Score::FullHouse,
            (Score::ThreeKind, 3) => Score::FourKind,
            (Score::ThreeKind, 1) => Score::FourKind,
            (Score::ThreeKind, _) => Score::ThreeKind,
            (Score::TwoPair, 2) => Score::FourKind,
            (Score::TwoPair, 1) => Score::FullHouse,
            (Score::TwoPair, _) => Score::TwoPair,
            (Score::OnePair, 2) => Score::ThreeKind,
            (Score::OnePair, 1) => Score::ThreeKind,
            (Score::OnePair, _) => Score::OnePair,
            (Score::HighCard, 1) => Score::OnePair,
            (Score::HighCard, _) => Score::HighCard,
        }
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
        match self.joker_score().cmp(&other.joker_score()) {
            Ordering::Equal => {
                let orderings = self
                    .cards
                    .iter()
                    .zip(other.cards.iter())
                    .filter_map(|(x_card, y_card)| match x_card.cmp(y_card) {
                        Ordering::Equal => None,
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
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
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

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let res = _input
        .trim()
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            let cards = cards
                .chars()
                .filter_map(|c| Card::try_from(c).ok())
                .collect_vec();
            Hand {
                cards: cards.try_into().expect("Should be exactly five cards"),
                bid: bid.parse().expect("Should parse into usize"),
            }
        })
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
        assert_eq!("5905", process(input)?);
        Ok(())
    }
}


