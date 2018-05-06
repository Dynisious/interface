
pub use std::convert::TryInto;
use std::fmt::{self, Display, Formatter};

mod unit;

pub use self::unit::*;

/// An `Entity` is some inhabitant of a `Tile`.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Entity {
    /// A `Unit` Entity.
    Unit(Box<Unit>),
}

impl Entity {
    pub fn as_byte(&self) -> u8 {
        match self {
            Entity::Unit(_) => b'U',
        }
    }
}

impl From<Box<Unit>> for Entity {
    fn from(from: Box<Unit>) -> Self {
        Entity::Unit(from)
    }
}

impl TryInto<Box<Unit>> for Entity {
    type Error = Self;

    fn try_into(self) -> Result<Box<Unit>, Self::Error> {
        // if let Entity::Unit(unit) = self { Ok(unit) } else { Err(self) }
        let Entity::Unit(res) = self; Ok(res)
    }
}

impl Display for Entity {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Entity::Unit(unit) => unit.fmt(fmt),
        }
    }
}
