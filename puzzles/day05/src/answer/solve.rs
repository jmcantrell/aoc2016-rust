use super::{Parsed1, Parsed2};

type Solution = String;
pub type Solution1 = Solution;
pub type Solution2 = Solution;

const PASSWORD_LENGTH: usize = 8;

pub fn solve1(door_id: &Parsed1) -> anyhow::Result<Solution1> {
    Ok(door_id
        .relevant_hashes()
        .map(|s| s.chars().nth(5).unwrap())
        .take(PASSWORD_LENGTH)
        .collect())
}

pub fn solve2(door_id: &Parsed2) -> anyhow::Result<Solution2> {
    let mut password: [Option<char>; PASSWORD_LENGTH] = [None; PASSWORD_LENGTH];
    let mut set_count = 0;

    for hash in door_id.relevant_hashes() {
        let c = hash.chars().nth(5).unwrap();
        if let Some(digit) = c.to_digit(10) {
            if let Some(slot) = password.get_mut(digit as usize) {
                if slot.is_none() {
                    slot.replace(hash.chars().nth(6).unwrap());
                    set_count += 1;
                    if set_count == PASSWORD_LENGTH {
                        break;
                    }
                }
            }
        }
    }

    Ok(password.into_iter().map(|c| c.unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use aoc::Input;

    use crate::answer::{parse1, parse2};

    const INPUT: Input = include_str!("../../input-test.txt");

    #[test]
    fn solve1() -> anyhow::Result<()> {
        assert_eq!(super::solve1(&parse1(INPUT)?)?, "18f47a30");
        Ok(())
    }

    #[test]
    fn solve2() -> anyhow::Result<()> {
        assert_eq!(super::solve2(&parse2(INPUT)?)?, "05ace8e3");
        Ok(())
    }
}
