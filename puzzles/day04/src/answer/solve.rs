use anyhow::Context;

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(room_codes: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(room_codes
        .iter()
        .filter(|code| code.is_real())
        .map(|code| code.sector_id)
        .sum())
}

pub fn solve2(room_codes: &Parsed2) -> anyhow::Result<Solution2> {
    room_codes
        .iter()
        .find_map(|code| {
            (code.decrypt_name() == "northpole object storage").then_some(code.sector_id)
        })
        .context("no solution")
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::parse1;

    use super::*;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT)?)?, 1514);
        Ok(())
    }
}
