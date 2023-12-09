use std::collections::{BinaryHeap, HashMap};

use crate::core::Messages;

use super::{Parsed1, Parsed2};

type Solution = String;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn columns_ordered_by_frequency(messages: &Messages) -> Vec<Vec<char>> {
    let counts_by_column: Vec<HashMap<char, usize>> = (0..messages[0].len())
        .map(|column| {
            (0..messages.len()).fold(HashMap::new(), |mut acc, row| {
                *acc.entry(messages[row][column]).or_default() += 1;
                acc
            })
        })
        .collect();

    let freqs_by_column: Vec<BinaryHeap<(usize, char)>> = counts_by_column
        .into_iter()
        .map(|counts| counts.into_iter().map(|(c, count)| (count, c)).collect())
        .collect();

    freqs_by_column
        .into_iter()
        .map(|mut freqs| {
            let mut column = Vec::with_capacity(freqs.len());
            while let Some((_, c)) = freqs.pop() {
                column.push(c);
            }
            column
        })
        .collect()
}

pub fn solve1(messages: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(columns_ordered_by_frequency(messages)
        .into_iter()
        .map(|column| column[0])
        .collect())
}

pub fn solve2(messages: &Parsed2) -> anyhow::Result<Solution2> {
    Ok(columns_ordered_by_frequency(messages)
        .into_iter()
        .map(|column| column[column.len() - 1])
        .collect())
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, "easter");
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT)?)?, "advent");
        Ok(())
    }
}
