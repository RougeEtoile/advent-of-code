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
    let mut current_node = graph
        .keys()
        .cloned()
        .collect_vec()
        .first()
        .unwrap()
        .to_owned();
    let res = instructs
        .iter()
        .cycle()
        .position(|direction| {
            let value = graph.get(current_node).expect("Node should be in map");
            current_node = match direction {
                Instruction::L => value.0,
                Instruction::R => value.1,
            };

            current_node == "ZZZ"
        })
        .map(|f| f + 1)
        .expect("Should have found solution");

    Ok(res.to_string())
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
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
