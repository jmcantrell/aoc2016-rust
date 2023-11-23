use anyhow::Context;

use aoc::Input;

use crate::core::{Instructions, Movement};

type Parsed = Instructions;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    fn parse_movement(s: &str) -> anyhow::Result<Movement> {
        let i = s
            .chars()
            .position(|c: char| c.is_ascii_digit())
            .context("expected string to end with one or more digits")?;

        let (turn, steps) = s.split_at(i);
        let turn = turn.try_into()?;
        let steps = steps.parse()?;

        Ok((turn, steps))
    }

    input
        .split(',')
        .enumerate()
        .map(|(i, s)| {
            parse_movement(s.trim()).with_context(|| format!("movement number: {}", i + 1))
        })
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

    const INPUT: Input = include_str!("../../input.txt");

    #[test]
    fn parse() -> anyhow::Result<()> {
        dbg!(super::parse(INPUT)?);
        Ok(())
    }
}
