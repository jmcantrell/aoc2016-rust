use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(addresses: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(addresses
        .iter()
        .filter(|address| address.supports_tls())
        .count())
}

pub fn solve2(addresses: &Parsed2) -> anyhow::Result<Solution2> {
    Ok(addresses
        .iter()
        .filter(|address| address.supports_ssl())
        .count())
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    use super::*;

    const INPUT1: Input = include_str!("../../input-test1.txt");
    const INPUT2: Input = include_str!("../../input-test2.txt");

    #[test]
    fn test_solve1() -> anyhow::Result<()> {
        assert_eq!(solve1(&parse1(INPUT1)?)?, 2);
        Ok(())
    }

    #[test]
    fn test_solve2() -> anyhow::Result<()> {
        assert_eq!(solve2(&parse2(INPUT2)?)?, 3);
        Ok(())
    }
}
