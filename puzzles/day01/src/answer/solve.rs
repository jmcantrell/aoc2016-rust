use anyhow::anyhow;

use std::collections::HashSet;

use crate::core::{Direction, Location};

use super::{Parsed1, Parsed2};

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn walk(location: Location, direction: Direction, steps: usize) -> Location {
    let (row, column) = location;

    match direction {
        Direction::North => (row.saturating_sub_unsigned(steps), column),
        Direction::South => (row.saturating_add_unsigned(steps), column),
        Direction::West => (row, column.saturating_sub_unsigned(steps)),
        Direction::East => (row, column.saturating_add_unsigned(steps)),
    }
}

fn manhattan_distance_from_origin((row, column): Location) -> usize {
    row.unsigned_abs() + column.unsigned_abs()
}

pub fn solve1(instructions: &Parsed1) -> anyhow::Result<Solution1> {
    let mut direction = Direction::default();
    let mut location = Location::default();

    for &(turn, steps) in instructions.iter() {
        direction += turn;
        location = walk(location, direction, steps);
    }

    Ok(manhattan_distance_from_origin(location))
}

pub fn solve2(instructions: &Parsed2) -> anyhow::Result<Solution2> {
    let mut direction = Direction::default();
    let mut location = Location::default();
    let mut visits: HashSet<Location> = Default::default();

    for &(turn, steps) in instructions.iter() {
        direction += turn;
        for _ in 0..steps {
            location = walk(location, direction, 1);
            if visits.contains(&location) {
                return Ok(manhattan_distance_from_origin(location));
            } else {
                visits.insert(location);
            }
        }
    }

    Err(anyhow!("no solution"))
}

#[cfg(test)]
mod tests {
    use crate::answer::{parse1, parse2};

    #[test]
    fn solve1() -> anyhow::Result<()> {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(super::solve1(&parse1($input)?)?, $expected);
            };
        }

        test!("R2, L3", 5);
        test!("R2, R2, R2", 2);
        test!("R5, L5, R5, R3", 12);

        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        macro_rules! test {
            ($input:expr, $expected:expr) => {
                assert_eq!(super::solve2(&parse2($input)?)?, $expected);
            };
        }

        test!("R8, R4, R4, R8", 4);

        Ok(())
    }
}
