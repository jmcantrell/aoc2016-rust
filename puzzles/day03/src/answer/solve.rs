use itertools::Itertools;

use crate::core::{Side, N};

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn is_triangle_possible(candidate: &[Side]) -> bool {
    candidate
        .iter()
        .permutations(N)
        .all(|mut sides| *sides.pop().unwrap() < sides.into_iter().sum())
}

pub fn solve1(grid: &Parsed1) -> anyhow::Result<Solution1> {
    Ok((0..grid.len())
        .map(|row| {
            (0..N)
                .map(move |column| grid[row][column])
                .collect::<Vec<_>>()
        })
        .filter(|candidate| is_triangle_possible(candidate))
        .count())
}

pub fn solve2(grid: &Parsed2) -> anyhow::Result<Solution2> {
    Ok((0..N)
        .flat_map(|column| {
            (0..grid.len()).step_by(N).map(move |i| {
                (i..i + N)
                    .map(move |row| grid[row][column])
                    .collect::<Vec<_>>()
            })
        })
        .filter(|candidate| is_triangle_possible(candidate))
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_triangle_possible() {
        macro_rules! assert_impossible {
            ($sides:expr) => {
                assert!(!is_triangle_possible(&$sides));
            };
        }

        assert_impossible!([5, 10, 25]);
    }
}
