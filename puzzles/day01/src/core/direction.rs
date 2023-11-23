use std::ops::{Add, AddAssign};

use super::Turn;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Default for Direction {
    fn default() -> Self {
        Self::North
    }
}

impl Add<Turn> for Direction {
    type Output = Self;

    fn add(self, turn: Turn) -> Self {
        use Direction::*;
        use Turn::*;

        match self {
            North => match turn {
                Left => West,
                Right => East,
            },
            South => match turn {
                Left => East,
                Right => West,
            },
            West => match turn {
                Left => South,
                Right => North,
            },
            East => match turn {
                Left => North,
                Right => South,
            },
        }
    }
}

impl AddAssign<Turn> for Direction {
    fn add_assign(&mut self, turn: Turn) {
        *self = *self + turn;
    }
}
