use itertools::Itertools;

use crate::custom_error::AocError;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Space {
    Galaxy,
    Empty,
}
impl TryFrom<char> for Space {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Space::Galaxy),
            '.' => Ok(Space::Empty),
            _ => Err("Invalid Galatic Body".to_string()),
        }
    }
}
#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let universe = _input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|x| x.try_into().ok())
                .collect::<Vec<Space>>()
        })
        .collect::<Vec<Vec<Space>>>();
    let empty_rows = universe
        .iter()
        .enumerate()
        .filter_map(|(y, row)| row.iter().all(|x| *x == Space::Empty).then_some(y))
        .collect::<Vec<usize>>();
    let empty_cols = (0..universe.first().cloned().unwrap().len())
        .filter(|x| {
            (0..universe.len()).all(|y| *universe.get(y).unwrap().get(*x).unwrap() == Space::Empty)
        })
        .collect::<Vec<usize>>();
    let galaxies = universe
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, val)| (*val == Space::Galaxy).then_some((x, y)))
        })
        .collect::<Vec<(usize, usize)>>();
    dbg!(&galaxies.len());
    let combos = galaxies
        .iter()
        .combinations(2)
        .map(|x| {
            distance(
                x.get(0).unwrap(),
                x.get(1).unwrap(),
                &empty_rows,
                &empty_cols,
            )
        })
        .sum::<usize>();
    dbg!(combos);
    Ok(combos.to_string())
}
fn distance(
    a: &(usize, usize),
    b: &(usize, usize),
    empty_rows: &[usize],
    empty_cols: &[usize],
) -> usize {
    let x_dist = match b.0 > a.0 {
        true => (b.0 - a.0) + (a.0..b.0).filter(|x| empty_cols.contains(x)).count(),
        _ => a.0 - b.0 + (b.0..a.0).filter(|x| empty_cols.contains(x)).count(),
    };
    let y_dist = match b.1 > a.1 {
        true => (b.1 - a.1) + (a.1..b.1).filter(|x| empty_rows.contains(x)).count(),
        _ => a.1 - b.1 + (b.1..a.1).filter(|x| empty_rows.contains(x)).count(),
    };
    x_dist + y_dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!("374", process(input)?);
        Ok(())
    }
}
