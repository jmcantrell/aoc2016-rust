use anyhow::{anyhow, ensure, Context};

use std::convert::TryFrom;

use super::Axis;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Rect {
        width: usize,
        height: usize,
    },
    Rotate {
        axis: Axis,
        index: usize,
        count: usize,
    },
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Rect { width, height } => write!(f, "rect {}x{}", width, height),
            Self::Rotate { axis, index, count } => write!(
                f,
                "rotate {} {}={} by {}",
                axis,
                axis.index_name(),
                index,
                count
            ),
        }
    }
}

impl TryFrom<&str> for Operation {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> anyhow::Result<Self> {
        let mut tokens = s.split_whitespace();

        let name = tokens.next().context("missing operation name")?;

        let operation = match name {
            "rect" => {
                let (left, right) = tokens
                    .next()
                    .context("missing rect size")?
                    .split_once('x')
                    .context("expected size to be delimited by an 'x'")?;

                let width = left.parse()?;
                let height = right.parse()?;

                Ok(Self::Rect { width, height })
            }
            "rotate" => {
                let axis: Axis = tokens.next().context("missing rotate axis")?.try_into()?;

                let (index_name, index_value) = tokens
                    .next()
                    .context("missing rotate index")?
                    .split_once('=')
                    .context("expected rotate index to be delimited by an '='")?;

                ensure!(
                    index_name == axis.index_name(),
                    "expected rotate index name to be {:?}, but it was {:?}",
                    axis.index_name(),
                    index_name
                );

                let index = index_value.parse()?;

                let token = tokens.next().context("rotate missing 'by'")?;

                ensure!(token == "by", "expected 'by' token, but got {:?}", token);

                let count = tokens.next().context("rotate missing count")?.parse()?;

                Ok(Self::Rotate { axis, index, count })
            }
            _ => Err(anyhow!("invalid operation: {:?}", s)),
        }?;

        ensure!(tokens.next().is_none(), "operation had unexpected tokens");

        Ok(operation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_str() {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(Operation::try_from($input).unwrap(), $expected);
            };
        }

        test!(
            "rect 3x2",
            Operation::Rect {
                width: 3,
                height: 2
            }
        );

        test!(
            "rotate row y=0 by 4",
            Operation::Rotate {
                axis: Axis::Row,
                index: 0,
                count: 4
            }
        );

        test!(
            "rotate column x=1 by 2",
            Operation::Rotate {
                axis: Axis::Column,
                index: 1,
                count: 2
            }
        );
    }
}
