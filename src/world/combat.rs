
use std::iter::{IntoIterator, Once, self};
use world::entities::Entity;

/// A `Combat` instance is a conflict between two `Entity`s.
pub struct Combat<'combat>(Entity, &'combat mut Entity);

impl<'combat> Combat<'combat> {
    pub fn new(attacker: Entity, defender: &'combat mut Entity) -> Self { Combat(attacker, defender) }
    /// Consume the instance and resolve it.
    /// 
    /// The attacker wins.
    pub fn resolve(self) -> bool { *self.1 = self.0; true }
}

impl<'combat> IntoIterator for Combat<'combat> {
    type Item = bool;
    type IntoIter = Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter { iter::once(self.resolve()) }
}
