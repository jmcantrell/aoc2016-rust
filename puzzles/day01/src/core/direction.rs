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
        match self {
            Self::North => match turn {
                Turn::Left => Self::West,
                Turn::Right => Self::East,
            },
            Self::South => match turn {
                Turn::Left => Self::East,
                Turn::Right => Self::West,
            },
            Self::West => match turn {
                Turn::Left => Self::South,
                Turn::Right => Self::North,
            },
            Self::East => match turn {
                Turn::Left => Self::North,
                Turn::Right => Self::South,
            },
        }
    }
}

impl AddAssign<Turn> for Direction {
    fn add_assign(&mut self, turn: Turn) {
        *self = *self + turn;
    }
}
