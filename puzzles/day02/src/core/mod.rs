pub mod direction;
pub use direction::*;

pub mod keypad;
pub use keypad::*;

pub type Location = (usize, usize);
pub type Instruction = Vec<Direction>;
pub type Instructions = Vec<Instruction>;
