use anyhow::{ensure, Context};

use aoc::Input;

use crate::core::{Side, N};

type Parsed = Vec<Vec<Side>>;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

pub fn parse(input: Input) -> anyhow::Result<Parsed> {
    fn parse_side_triplet(s: &str) -> anyhow::Result<Vec<Side>> {
        let sides = s
            .split_whitespace()
            .enumerate()
            .map(|(i, s)| s.parse().with_context(|| format!("side number {}", i + 1)))
            .collect::<Result<Vec<_>, _>>()?;

        ensure!(sides.len() == N, "expected {}, but got {}", N, sides.len());

        Ok(sides)
    }

    input
        .lines()
        .enumerate()
        .map(|(i, s)| parse_side_triplet(s).with_context(|| format!("triplet number {}", i + 1)))
        .collect::<Result<Vec<_>, _>>()
}

pub fn parse1(input: Input) -> anyhow::Result<Parsed1> {
    parse(input)
}

pub fn parse2(input: Input) -> anyhow::Result<Parsed2> {
    parse(input)
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use super::*;

    const INPUT: Input = include_str!("../../input.txt");

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        dbg!(parse(INPUT)?);
        Ok(())
    }
}
