use anyhow::anyhow;
use lazy_regex::{lazy_regex, Lazy, Regex};

use std::convert::TryFrom;

use super::{Index, Recipient, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Input {
        value: Value,
        bot: Index,
    },
    Output {
        bot: Index,
        recipient_low: Recipient,
        recipient_high: Recipient,
    },
}

impl TryFrom<&str> for Instruction {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> anyhow::Result<Self> {
        static RE_INPUT: Lazy<Regex> =
            lazy_regex!(r"^value (?<value>\d+) goes to bot (?<bot>\d+)$");

        static RE_OUTPUT: Lazy<Regex> = lazy_regex!(
            r"^bot (?<bot>\d+) gives low to (?<recipient_low>\w+ \d+) and high to (?<recipient_high>\w+ \d+)$"
        );

        if let Some(captures) = RE_INPUT.captures(s) {
            Ok(Self::Input {
                value: captures["value"].parse()?,
                bot: captures["bot"].parse()?,
            })
        } else if let Some(captures) = RE_OUTPUT.captures(s) {
            Ok(Self::Output {
                bot: captures["bot"].parse()?,
                recipient_low: captures["recipient_low"].try_into()?,
                recipient_high: captures["recipient_high"].try_into()?,
            })
        } else {
            return Err(anyhow!("invalid instruction: {:?}", s));
        }
    }
}
