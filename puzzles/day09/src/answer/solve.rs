use aoc::Input;

use crate::core::decompressed_size;

type Solution = usize;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

pub fn solve1(parsed: Input) -> anyhow::Result<Solution1> {
    Ok(decompressed_size(parsed.chars(), false))
}

pub fn solve2(parsed: Input) -> anyhow::Result<Solution2> {
    Ok(decompressed_size(parsed.chars(), true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        macro_rules! test {
            ($input:expr => $expected:expr) => {
                assert_eq!(solve1($input).unwrap(), $expected);
            };
        }

        test!("ADVENT" => 6);
        test!("A(1x5)BC" => 7);
        test!("(3x3)XYZ" => 9);
        test!("A(2x2)BCD(2x2)EFG" => 11);
        test!("(6x1)(1x3)A" => 6);
        test!("X(8x2)(3x3)ABCY" => 18);
    }

    #[test]
    fn test_solve2() {
        macro_rules! test {
            ($input:expr => $expected:expr) => {
                assert_eq!(solve2($input).unwrap(), $expected);
            };
        }

        test!("(3x3)XYZ" => 9);
        test!("X(8x2)(3x3)ABCY" => 20);
        test!("(27x12)(20x12)(13x14)(7x10)(1x12)A" => 241920);
        test!("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN" => 445);
    }
}
