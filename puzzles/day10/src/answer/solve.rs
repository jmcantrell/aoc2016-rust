use anyhow::anyhow;

use crate::core::{Factory, Index, Value};

use super::{Parsed1, Parsed2};

type Solution = Index;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

#[cfg(test)]
const TARGET_VALUES: [Value; 2] = [2, 5];

#[cfg(not(test))]
const TARGET_VALUES: [Value; 2] = [17, 61];

pub fn solve1(instructions: &Parsed1) -> anyhow::Result<Solution1> {
    let mut factory = Factory::new(instructions);

    while let Some(transfer) = factory.transfer() {
        if transfer.values == TARGET_VALUES {
            return Ok(transfer.origin);
        }
    }

    Err(anyhow!("no solution"))
}

pub fn solve2(instructions: &Parsed2) -> anyhow::Result<Solution2> {
    let mut factory = Factory::new(instructions);

    while factory.transfer().is_some() {}

    Ok(factory
        .outputs
        .into_iter()
        .take(3)
        .map(|mut output| output.pop().unwrap() as usize)
        .product())
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 2);
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, 30);
        Ok(())
    }
}
