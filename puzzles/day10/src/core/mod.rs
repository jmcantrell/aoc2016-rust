pub type Value = u8;
pub type Index = usize;

pub mod recipient;
pub use recipient::*;

pub mod instruction;
pub use instruction::*;

pub mod bot;
pub use bot::*;

pub mod factory;
pub use factory::*;
