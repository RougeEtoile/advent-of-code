use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Not,
};

use crate::custom_error::AocError;

// TryFrom char to enum that represents pipe directions, ground "." and starting space "S"
// create BTreeMap (x,y), pipe
// find position of starting space by getting key of value that equals "S"
// iter through connecting positions filter out ground
// recurse that until you reach starting space returning param2 + 1
// then return floor div 2 that number
// TODO: SOLVE THIS ITS GRAPH PRACTICE
// TODO: Part 2 involves Flood Fill
#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let graph = _input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, val)| ((x as i32, y as i32), Pipe::try_from(val).unwrap()))
                .collect::<Vec<((i32, i32), Pipe)>>()
        })
        .collect::<BTreeMap<(i32, i32), Pipe>>();
    let start_pos = graph
        .iter()
        .find_map(|x| (*x.1 == Pipe::Starting).then_some(*x.0))
        .expect("Should have starting piece");

    let (mut start, mut end) = next_two(start_pos, &graph).into();

    let mut seen = BTreeSet::new();
    seen.insert(start_pos);
    seen.insert(start);
    seen.insert(end);
    loop {
        start = next_pos(start, &graph, &seen);
        end = next_pos(end, &graph, &seen);
        if start != end {
            seen.insert(start);
            seen.insert(end);
        } else {
            break;
        }
    }
    let res = (seen.len() + 1) / 2;
    Ok(res.to_string())
}
fn next_pos(
    pos: (i32, i32),
    graph: &BTreeMap<(i32, i32), Pipe>,
    seen: &BTreeSet<(i32, i32)>,
) -> (i32, i32) {
    let piece = graph.get(&pos).expect("We already know this is valid");
    dbg!(pos);
    dbg!(piece);
    let next = match piece {
        Pipe::EW => vec![(pos.0 - 1, pos.1), (pos.0 + 1, pos.1)],
        Pipe::NS => vec![(pos.0, pos.1 + 1), (pos.0, pos.1 - 1)],
        Pipe::NE => vec![(pos.0 + 1, pos.1), (pos.0, pos.1 - 1)],
        Pipe::NW => vec![(pos.0 - 1, pos.1), (pos.0, pos.1 - 1)],
        Pipe::SW => vec![(pos.0 - 1, pos.1), (pos.0, pos.1 + 1)],
        Pipe::SE => vec![(pos.0 + 1, pos.1), (pos.0, pos.1 + 1)],
        Pipe::Ground => todo!(),
        Pipe::Starting => todo!(),
    };
    next.iter()
        .find_map(|x| seen.contains(x).not().then_some(*x))
        .expect("Go the direction we haven't seen")
}
fn next_two((x, y): (i32, i32), graph: &BTreeMap<(i32, i32), Pipe>) -> [(i32, i32); 2] {
    let mut beginnning_two = Vec::new();
    if graph
        .get(&(x + 1, y))
        .is_some_and(|pipe| matches!(*pipe, Pipe::EW | Pipe::NW | Pipe::SW))
    {
        beginnning_two.push((x + 1, y));
    };
    if graph
        .get(&(x - 1, y))
        .is_some_and(|pipe| matches!(*pipe, Pipe::EW | Pipe::NE | Pipe::SE))
    {
        beginnning_two.push((x - 1, y));
    };
    if graph
        .get(&(x, y - 1))
        .is_some_and(|pipe| matches!(*pipe, Pipe::NW | Pipe::NE | Pipe::NS))
    {
        beginnning_two.push((x, y - 1));
    };
    if graph
        .get(&(x, y + 1))
        .is_some_and(|pipe| matches!(*pipe, Pipe::NS | Pipe::SE | Pipe::SW))
    {
        beginnning_two.push((x, y + 1));
    };
    dbg!(&beginnning_two);

    beginnning_two
        .try_into()
        .expect("Should be two valid directions")
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Starting,
}
impl TryFrom<char> for Pipe {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Pipe::NS),
            '-' => Ok(Pipe::EW),
            'L' => Ok(Pipe::NE),
            'J' => Ok(Pipe::NW),
            '7' => Ok(Pipe::SW),
            'F' => Ok(Pipe::SE),
            '.' => Ok(Pipe::Ground),
            'S' => Ok(Pipe::Starting),
            _ => Err("Invalid Character".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!("4", process(input)?);
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        assert_eq!("8", process(input)?);
        Ok(())
    }
}
