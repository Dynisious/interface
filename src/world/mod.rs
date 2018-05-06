
pub mod entities;
mod combat;
mod worldpool;

pub use position::Pos;
pub use self::combat::*;
pub use self::worldpool::*;

pub fn initialise() -> WorldPool { WorldPool::default() }
