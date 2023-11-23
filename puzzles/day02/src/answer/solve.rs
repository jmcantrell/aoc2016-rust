use crate::core::{Direction, Instructions, Location, KEYPAD9, KEYPAD_WTF};

use super::{Parsed1, Parsed2};

type Solution = String;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn step(&(row, column): &Location, direction: &Direction) -> Location {
    match direction {
        Direction::Up => (row.saturating_sub(1), column),
        Direction::Down => (row.saturating_add(1), column),
        Direction::Left => (row, column.saturating_sub(1)),
        Direction::Right => (row, column.saturating_add(1)),
    }
}

fn solve(
    instructions: &Instructions,
    start: Location,
    get: fn(Location) -> Option<char>,
) -> String {
    let mut location = start;
    let mut buttons: Vec<char> = Vec::new();

    for instruction in instructions {
        for direction in instruction {
            let next_location = step(&location, direction);
            if get(next_location).is_some() {
                location = next_location;
            }
        }
        buttons.push(get(location).unwrap())
    }

    buttons.into_iter().collect()
}

pub fn solve1(instructions: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(solve(instructions, (1, 1), |location| {
        KEYPAD9.get(location).copied()
    }))
}

pub fn solve2(instructions: &Parsed2) -> anyhow::Result<Solution2> {
    Ok(solve(instructions, (2, 0), |location| {
        KEYPAD_WTF
            .get(location)
            .and_then(|maybe_value| maybe_value.as_ref())
            .copied()
    }))
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, "1985");
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, "5DB3");
        Ok(())
    }
}
