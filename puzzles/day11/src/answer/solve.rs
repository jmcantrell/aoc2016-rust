use anyhow::Context;

use crate::core::{min_moves_to_top, Component, ComponentKind};

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(floors: &Parsed1) -> anyhow::Result<Solution1> {
    min_moves_to_top(floors.clone()).context("no solution")
}

pub fn solve2(floors: &Parsed2) -> anyhow::Result<Solution2> {
    let mut floors_plus = floors.clone();

    floors_plus[0].move_in(["elerium", "dilithium"].into_iter().flat_map(|material| {
        [ComponentKind::Generator, ComponentKind::Microchip]
            .into_iter()
            .map(|kind| Component::new(material, kind))
    }));

    min_moves_to_top(floors_plus).context("no solution")
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::parse1;

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 11);
        Ok(())
    }
}
