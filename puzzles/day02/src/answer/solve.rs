use anyhow::Context;

use crate::core::{Direction, Instructions, Keypad, Location, KEYPAD9, KEYPAD_WTF};

use super::{Parsed1, Parsed2};

type Solution = String;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn step((row, column): Location, direction: Direction) -> Location {
    match direction {
        Direction::Up => (row.saturating_sub(1), column),
        Direction::Down => (row.saturating_add(1), column),
        Direction::Left => (row, column.saturating_sub(1)),
        Direction::Right => (row, column.saturating_add(1)),
    }
}

fn solve<const S: usize>(
    keypad: &Keypad<S>,
    instructions: &Instructions,
) -> anyhow::Result<Solution> {
    let start_button = '5';
    let mut location = keypad
        .find_button(start_button)
        .with_context(|| format!("no {:?} button", start_button))?;

    let mut buttons: Vec<char> = Vec::new();

    for instruction in instructions {
        for &direction in instruction {
            let next_location = step(location, direction);
            if keypad.get(next_location).is_some() {
                location = next_location;
            }
        }
        buttons.push(*keypad.get(location).unwrap());
    }

    Ok(buttons.into_iter().collect())
}

pub fn solve1(instructions: &Parsed1) -> anyhow::Result<Solution1> {
    solve(&KEYPAD9, instructions)
}

pub fn solve2(instructions: &Parsed2) -> anyhow::Result<Solution2> {
    solve(&KEYPAD_WTF, instructions)
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
