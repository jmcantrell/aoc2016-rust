pub type Value = isize;

pub mod register;
pub use register::*;

pub mod instruction;
pub use instruction::*;

pub mod computer;
pub use computer::*;
