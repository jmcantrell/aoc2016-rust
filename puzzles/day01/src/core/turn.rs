use anyhow::{anyhow, ensure};

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
        ensure!(
            s.len() == 1,
            "expected a single character, but there were {}",
            s.len()
        );

        s.chars().next().unwrap().try_into()
    }
}
