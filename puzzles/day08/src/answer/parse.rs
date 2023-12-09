use anyhow::Context;

use aoc::Input;

use crate::core::Operation;

pub type Parsed = Vec<Operation>;

pub fn parse(input: Input) -> anyhow::Result<Parsed> {
    input
        .lines()
        .enumerate()
        .map(|(i, s)| {
            s.try_into()
                .with_context(|| format!("operation number {}", i + 1))
        })
        .collect::<Result<_, _>>()
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
