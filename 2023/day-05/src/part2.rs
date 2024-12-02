use crate::custom_error::AocError;
use std::ops::Range;
use winnow::{
    ascii::{digit1, line_ending, space1},
    combinator::{preceded, separated, separated_pair},
    prelude::*,
    token::take_until0,
};
#[derive(Debug, PartialEq)]
pub struct Ecology {
    seeds: Vec<Range<usize>>,
    // Should be Vec of Ranges
    maps: Vec<Vec<Mapper>>,
}
#[derive(Debug, PartialEq)]
pub struct Mapper {
    range: Range<usize>,
    destination: usize,
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut input = _input;
    let eco = parse_ecology(&mut input).unwrap();
    let res = eco
        .seeds
        .iter()
        .map(|seeds: &Range<_>| {
            seeds
                .clone()
                .map(|seed| {
                    eco.maps
                        .iter()
                        .fold(seed, |acc: usize, mapper: &Vec<Mapper>| {
                            match &mapper.iter().find(|x: &&Mapper| x.range.contains(&acc)) {
                                Some(m) => acc - m.range.start + m.destination,
                                None => acc,
                            }
                        })
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    Ok(res.to_string())
}
pub fn parse_ecology(input: &mut &str) -> PResult<Ecology> {
    let seeds = parse_vec.parse_next(input)?;
    let maps = preceded(line_ending, parse_location_arr).parse_next(input)?;
    Ok(Ecology { seeds, maps })
}
pub fn parse_usize(input: &mut &str) -> PResult<usize> {
    digit1
        .try_map(|x: &str| x.parse::<usize>())
        .parse_next(input)
}
pub fn parse_vec(input: &mut &str) -> PResult<Vec<Range<usize>>> {
    preceded(
        ("seeds:", space1),
        separated(
            1..,
            separated_pair(parse_usize, space1, parse_usize).map(|x: (usize, usize)| Range {
                start: x.0,
                end: x.0 + x.1,
            }),
            space1,
        ),
    )
    .parse_next(input)
}
pub fn parse_location_arr(input: &mut &str) -> PResult<Vec<Vec<Mapper>>> {
    separated(1.., preceded(line_ending, parse_location), line_ending).parse_next(input)
}
pub fn parse_location(input: &mut &str) -> PResult<Vec<Mapper>> {
    preceded(
        (take_until0(":"), ":"),
        preceded(line_ending, separated(1.., parse_maps, line_ending)),
    )
    .parse_next(input)
}
pub fn parse_maps(input: &mut &str) -> PResult<Mapper> {
    (
        digit1.try_map(|x: &str| x.parse::<usize>()),
        preceded(space1, digit1.try_map(|x: &str| x.parse::<usize>())),
        preceded(space1, digit1.try_map(|x: &str| x.parse::<usize>())),
    )
        .map(|x: (usize, usize, usize)| Mapper {
            range: std::ops::Range {
                start: x.1,
                end: x.1 + x.2,
            },
            destination: x.0,
        })
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        // assert_eq!("35", process("seeds: 79 14 55 13")?);
        assert_eq!("46", process(input)?);
        Ok(())
    }
}


