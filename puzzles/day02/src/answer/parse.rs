use anyhow::Context;

use aoc::Input;

use crate::core::{Instruction, Instructions};

type Parsed = Instructions;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    fn parse_instructions(s: &str) -> anyhow::Result<Instruction> {
        s.chars()
            .enumerate()
            .map(|(i, c)| {
                c.try_into()
                    .with_context(|| format!("move number {}", i + 1))
            })
            .collect::<Result<Vec<_>, _>>()
    }

    input
        .lines()
        .enumerate()
        .map(|(i, s)| {
            parse_instructions(s).with_context(|| format!("instructions number {}", i + 1))
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

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        dbg!(parse(INPUT)?);
        Ok(())
    }
}
