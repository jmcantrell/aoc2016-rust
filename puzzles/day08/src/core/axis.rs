use anyhow::anyhow;

use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Row,
    Column,
}

impl Axis {
    pub fn index_name(&self) -> &str {
        match self {
            Self::Row => "y",
            Self::Column => "x",
        }
    }
}

impl std::fmt::Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Row => "row",
                Self::Column => "column",
            }
        )
    }
}

impl TryFrom<&str> for Axis {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> anyhow::Result<Self> {
        match s {
            "row" => Ok(Self::Row),
            "column" => Ok(Self::Column),
            _ => Err(anyhow!("invalid axis: {:?}", s)),
        }
    }
}
