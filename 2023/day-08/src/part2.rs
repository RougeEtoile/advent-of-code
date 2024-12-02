use std::collections::BTreeMap;

use crate::custom_error::AocError;
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{alpha1, line_ending, space1};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, separated_pair, tuple};
use nom::{IResult, Parser};
use nom_supreme::parser_ext::ParserExt;
use nom_supreme::tag::complete::tag;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let (_inp, (instructs, graph)) = parse_input(_input).expect("Parser worked");
    dbg!(_inp);
    let ghosts = graph
        .keys()
        .filter(|&key| key.ends_with('A'))
        .cloned()
        .collect_vec();
    let res = ghosts
        .iter()
        .map(|node| {
            let mut current_node = *node;
            instructs
                .iter()
                .cycle()
                .position(|direction| {
                    let value = graph.get(current_node).expect("Node should be in map");
                    let next_node = match direction {
                        Instruction::L => value.0,
                        Instruction::R => value.1,
                    };
                    if next_node.ends_with('Z') {
                        true
                    } else {
                        current_node = next_node;
                        false
                    }
                })
                .map(|f| f + 1)
                .expect("Must have solution")
        })
        .collect_vec();

    Ok(least_common_multiple(&res).to_string())
}
fn least_common_multiple(cycles: &[usize]) -> usize {
    if cycles.len() == 1 {
        cycles[0]
    } else {
        let x = cycles[0];
        let y = least_common_multiple(&cycles[1..]);
        x * y / greatest_common_divisor(x, y)
    }
}
fn greatest_common_divisor(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        greatest_common_divisor(y, x % y)
    }
}
#[derive(Debug, Clone)]
enum Instruction {
    L,
    R,
}
fn parse_instruction(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(alt((
        tag("L").value(Instruction::L),
        tag("R").value(Instruction::R),
    )))
    .parse(input)
}
fn parse_graph(input: &str) -> IResult<&str, BTreeMap<&str, (&str, &str)>> {
    let (inp, lines) = separated_list1(
        line_ending,
        tuple((
            alpha1.terminated(tuple((space1, tag("="), space1))),
            delimited(
                tag("("),
                separated_pair(alpha1.terminated(tag(",")), space1, alpha1),
                tag(")"),
            ),
        )),
    )
    .parse(input)?;
    Ok((
        inp,
        lines.iter().fold(BTreeMap::new(), |mut acc, line| {
            acc.insert(line.0, line.1);
            acc
        }),
    ))
}
fn parse_input(input: &str) -> IResult<&str, (Vec<Instruction>, BTreeMap<&str, (&str, &str)>)> {
    tuple((
        parse_instruction.terminated(line_ending),
        line_ending.precedes(parse_graph),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}


