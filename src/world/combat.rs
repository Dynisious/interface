
use world::entities::Entity;

pub struct Combat<'a>(Entity, &'a mut Entity);

impl<'a> Combat<'a> {
    pub fn new(attacker: Entity, defender: &'a mut Entity) -> Self { Combat(attacker, defender) }
    pub fn resolve(self) { *self.1 = self.0 }
}
