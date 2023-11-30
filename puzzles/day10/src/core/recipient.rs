use anyhow::{anyhow, ensure, Context};

use std::convert::TryFrom;

use super::Index;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Recipient {
    Bot(Index),
    Output(Index),
}

impl TryFrom<&str> for Recipient {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> anyhow::Result<Self> {
        let mut tokens = s.split_whitespace();

        let key = tokens.next().context("missing target name")?;
        let index = tokens.next().context("missing target index")?.parse()?;

        let target = match key {
            "bot" => Ok(Self::Bot(index)),
            "output" => Ok(Self::Output(index)),
            _ => Err(anyhow!("invalid target: {:?}", s)),
        }?;

        ensure!(tokens.next().is_none(), "unexpected trailing tokens");

        Ok(target)
    }
}
