
use std::fmt::{Display, Formatter, Result};

/// A `Unit` is a combatant.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Unit;

impl Display for Unit {
    fn fmt(&self, fmt: &mut Formatter) -> Result { write!(fmt, "U") }
}
