
use std::collections::{HashMap, hash_map::Entry::{Occupied, Vacant}};
use std::ops::Deref;
use world::{Pos, entities::Entity, Combat};

/// A `WorldPool` is a collection of `Entitys` mapped to positions.
pub struct WorldPool(HashMap<Pos, Entity>);

impl WorldPool {
    pub fn new(world: HashMap<Pos, Entity>) -> Self { WorldPool(world) }
    /// Attempts to add `entity` to the `WorldPool`.
    /// 
    /// Tries to insert `entity` at `dest`; if the space is already filled `entity` is returned.
    /// 
    /// # Params
    /// 
    /// dest --- The `Pos` to insert `entity` at.
    /// entity --- The `Entity` to insert.
    pub fn add_to(&mut self, dest: Pos, entity: Entity) -> Result<(), Entity> {
        //Check if an `Entity` exists.
        if let Vacant(entry) = self.0.entry(dest) {
            //The position is vacant.
            entry.insert(entity); Ok(())
        } else {
            //The position is occupied.
            Err(entity)
        }
    }
    /// Places `entity` into the `WorldPool`.
    /// 
    /// If the location `entity` is being placed is occupied, the `WorldPool` is locked
    /// until combat is resolved.
    /// 
    /// # Params
    /// 
    /// dest --- The `Pos` to place `entity` at.
    /// entity --- The `Entity` to place.
    pub fn move_to(&mut self, dest: Pos, entity: Entity) -> Result<(), Combat> {
        //Insert the `entity` or initiate combat.
        match self.0.entry(dest) {
            //The entry is empty, insert `entity`.
            Vacant(entry) => { entry.insert(entity); Ok(()) },
            //Initiate combat, create a new `Combat` instance to be resolved.
            Occupied(entry) => Err(Combat::new(entity, entry.into_mut())),
        }
    }
    /// Attempts to extract an `Entity` from the `WorldPool`.
    /// 
    /// If an `Entity` is found at `pos`, it is removed and returned.
    /// 
    /// # Params
    /// 
    /// pos --- The `Pos` to remove at.
    pub fn remove_from(&mut self, pos: &Pos) -> Option<Entity> {
        //Remove the `Entity` at `pos`.
        self.0.remove(pos)
    }
}

impl Default for WorldPool {
    fn default() -> Self {
        WorldPool::new(
            [Pos::new(0, 0), Pos::new(0, 3), Pos::new(3, 2)].iter()
            .map(|pos| (pos.clone(), Entity::from(Box::new(::world::entities::Unit))))
            .collect::<HashMap<_, _>>()
        )
    }
}

impl Deref for WorldPool {
    type Target = HashMap<Pos, Entity>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use world::entities::Unit;

    #[test]
    fn test_worldpool() {
        let mut pool = WorldPool::default();
        let pos = Pos::new(3, 4);
        let entity = Entity::from(Box::new(Unit));

        //Fill `pos` in `pool`.
        if let Err(_) = pool.add_to(pos, entity.clone()) {
            //There was an error while inserting into `pool`.
            panic!("`WorldPool::add_to` failed.")
        }
        //Check the insertion succeeded.
        assert_ne!(pool.get(&pos), None, "`WorldPool::add_to` failed to insert the `Entity`.");

        //Attempt to move into occupied `pos`.
        match pool.move_to(pos, entity.clone()) {
            //There should have been a combat initiated.
            Ok(_) => panic!("`WorldPool::move_to` failed to initiate `Combat`."),
            //Combat was initiated, resolve it.
            Err(lock) => assert!(lock.resolve(), "`WorldPool::move_to` combat failed."),
        }

        //Remove the `Entity` from `pos`.
        if let None = pool.remove_from(&pos) {
            //There was nothing to remove.
            panic!("`WorldPool::remove_from` failed.")
        }
    }
}
