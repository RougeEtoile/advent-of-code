use std::collections::HashMap;

use itertools::Itertools;

use crate::custom_error::AocError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}
impl TryFrom<char> for Spring {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Spring::Operational),
            '#' => Ok(Spring::Damaged),
            '?' => Ok(Spring::Unknown),
            _ => Err("Invalid character in input".to_string()),
        }
    }
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let res = _input
        .lines()
        .map(|line| {
            let (first_split, second_split) = line.split_once(' ').unwrap();
            let springs: Vec<Spring> = first_split
                .chars()
                .map(|value| Spring::try_from(value).unwrap())
                .collect();

            let conditions = second_split
                .chars()
                .filter_map(|x| x.to_digit(10))
                .map(|x| x as usize)
                .counts()
                .into_iter()
                .sorted_by_key(|f| f.0)
                .rev()
                .collect_vec();
            dbg!(&conditions);
            // Conditions is a list of counts
            // fold over and for each create a new vec of springs for every valid permutation and
            // then iterate over those permuations with next iteration of counts
            conditions
                .iter()
                .fold(vec![springs.clone()], |acc, &(count, occurence)| {
                    let mut i = 0_usize;
                    while i + count < springs.len() {
                        let a = &springs[i..count];
                        match a
                            .iter()
                            .all(|f| *f == Spring::Damaged || *f == Spring::Unknown)
                        {
                            true => {}
                            false => {}
                        }
                        i += 1;
                    }
                });

            // Must be be operational square after a block damaged
            //
            10
        })
        .collect::<Vec<usize>>();
    Ok(res.first().unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
