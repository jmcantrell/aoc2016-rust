use crate::core::{Computer, Instruction, Register, Value};

use super::{Parsed1, Parsed2};

type Solution = Value;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

fn solve(mut computer: Computer, instructions: &[Instruction]) -> Value {
    computer.run(instructions);
    computer.get(Register::A)
}

pub fn solve1(instructions: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(solve(Computer::new(), instructions))
}

pub fn solve2(instructions: &Parsed2) -> anyhow::Result<Solution2> {
    let mut computer = Computer::new();
    computer.set(Register::C, 1);

    Ok(solve(computer, instructions))
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::parse1;

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, 42);
        Ok(())
    }
}
