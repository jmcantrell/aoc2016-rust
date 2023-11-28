use aoc::{Input, Parse, Solve};

use day08::aoc::{Parser, Solver};

const INPUT: Input = include_str!("../input.txt");

fn main() -> anyhow::Result<()> {
    println!("Solution: {:?}", Solver(Parser(INPUT).parse()?).solve()?);
    Ok(())
}
