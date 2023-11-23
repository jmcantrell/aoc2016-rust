pub mod turn;
pub use turn::*;

pub mod direction;
pub use direction::*;

pub type Location = (isize, isize);
pub type Movement = (Turn, usize);
pub type Instructions = Vec<Movement>;
