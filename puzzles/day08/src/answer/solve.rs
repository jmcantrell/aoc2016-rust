use crate::core::Screen;

use super::Parsed;

pub type Solution = usize;

#[cfg(not(test))]
pub const SCREEN_SIZE: (usize, usize) = (50, 6);

#[cfg(test)]
pub const SCREEN_SIZE: (usize, usize) = (7, 3);

pub fn solve(operations: &Parsed) -> anyhow::Result<Solution> {
    let (width, height) = SCREEN_SIZE;
    let mut screen = Screen::new(width, height);

    for operation in operations {
        screen.apply(operation);
    }

    println!("{}", screen);

    Ok(screen.lit_count())
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::parse;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve() -> anyhow::Result<()> {
        assert_eq!(super::solve(&parse(INPUT)?)?, 6);
        Ok(())
    }
}
