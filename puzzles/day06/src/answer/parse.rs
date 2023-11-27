use anyhow::{ensure, Context};

use aoc::Input;

use crate::core::Messages;

type Parsed = Messages;
pub type Parsed1 = Parsed;
pub type Parsed2 = Parsed;

fn parse(input: Input) -> anyhow::Result<Parsed> {
    let rows = input
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut lens = rows.iter().map(|row| row.len());
    let expected_len = lens.next().context("empty input")?;

    for (i, len) in lens.enumerate() {
        ensure!(
            len == expected_len,
            "expected line {} length to be {}, but it was {}",
            i + 2,
            expected_len,
            len
        );
    }

    Ok(rows)
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

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn parse1() -> anyhow::Result<()> {
        dbg!(super::parse1(INPUT)?);
        Ok(())
    }

    #[test]
    fn parse2() -> anyhow::Result<()> {
        dbg!(super::parse2(INPUT)?);
        Ok(())
    }
}
