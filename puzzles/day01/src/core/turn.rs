use anyhow::{anyhow, ensure, Context};

use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Turn {
    Left,
    Right,
}

impl TryFrom<char> for Turn {
    type Error = anyhow::Error;

    fn try_from(c: char) -> anyhow::Result<Self> {
        match c {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(anyhow!("invalid turn: {:?}", c)),
        }
    }
}

impl TryFrom<&str> for Turn {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> anyhow::Result<Self> {
        let mut chars = s.chars();

        let turn = chars.next().context("string is empty")?.try_into()?;

        let rest_count = chars.count();

        ensure!(
            rest_count == 0,
            "expected a single character, but there were {}",
            rest_count
        );

        Ok(turn)
    }
}
