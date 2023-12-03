use anyhow::{anyhow, Context};

use std::convert::TryFrom;

use super::{Register, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input {
    Value(Value),
    Register(Register),
}

impl TryFrom<&str> for Input {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> anyhow::Result<Self> {
        if let Ok(value) = s.parse() {
            Ok(Self::Value(value))
        } else {
            Ok(Self::Register(s.try_into()?))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Copy(Input, Register),
    Increment(Register),
    Decrement(Register),
    JumpNonZero(Input, Value),
}

impl TryFrom<&str> for Instruction {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> anyhow::Result<Self> {
        let mut tokens = s.split_whitespace();

        let key = tokens.next().context("empty input")?;

        match key {
            "cpy" => {
                let input = tokens
                    .next()
                    .context("missing input value/register")?
                    .try_into()?;

                let register = tokens
                    .next()
                    .context("missing output register")?
                    .try_into()?;

                Ok(Self::Copy(input, register))
            }
            "inc" => {
                let register = tokens.next().context("missing register")?.try_into()?;

                Ok(Self::Increment(register))
            }
            "dec" => {
                let register = tokens.next().context("missing register")?.try_into()?;

                Ok(Self::Decrement(register))
            }
            "jnz" => {
                let input = tokens
                    .next()
                    .context("missing input value/register")?
                    .try_into()?;

                let offset = tokens.next().context("missing offset")?.parse()?;

                Ok(Self::JumpNonZero(input, offset))
            }
            _ => Err(anyhow!("invalid instruction: {:?}", s)),
        }
    }
}
