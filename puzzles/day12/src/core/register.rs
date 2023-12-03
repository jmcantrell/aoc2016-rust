use anyhow::{anyhow, ensure, Context};

use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Register {
    A,
    B,
    C,
    D,
}

impl Register {
    pub fn index(&self) -> usize {
        match self {
            Self::A => 0,
            Self::B => 1,
            Self::C => 2,
            Self::D => 3,
        }
    }
}

impl TryFrom<char> for Register {
    type Error = anyhow::Error;

    fn try_from(c: char) -> anyhow::Result<Self> {
        match c {
            'a' => Ok(Self::A),
            'b' => Ok(Self::B),
            'c' => Ok(Self::C),
            'd' => Ok(Self::D),
            _ => Err(anyhow!("invalid register: {:?}", c)),
        }
    }
}

impl TryFrom<&str> for Register {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> anyhow::Result<Self> {
        let mut chars = s.chars();
        let c = chars.next().context("empty input")?;

        ensure!(chars.next().is_none(), "too many characters");

        c.try_into()
    }
}
