
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::Deref;
use position::*;
use entities::Entity;

mod tile;
mod tilepool_error;

pub use self::tile::*;
pub use self::tilepool_error::*;

/// A `TilePool` is a collection of `Tiles` mapped to positions.
pub struct TilePool(HashMap<Pos, Tile>);

impl TilePool {
    /// The minimum number of tiles for a `TilePool`.
    pub const MINIMUM_TILES: usize = Tile::NEIGHBOURS + 1;

    pub fn connect_neighbours(mut neighbours: [Pos; Self::MINIMUM_TILES]) -> Result<[(Pos, NeighbourArray); Self::MINIMUM_TILES], TilePoolError> {
        //Sort the neighbours for duplicate checking.
        neighbours.sort_unstable_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));

        //Check for duplicates.
        for index in 1..Self::MINIMUM_TILES {
            if neighbours[index] == neighbours[index - 1] { return Err(TilePoolError::RepeatedNeighbour(neighbours[index])) }
        }

        //Allocate memory for the result.
        let mut res: [(Pos, NeighbourArray); Self::MINIMUM_TILES] = unsafe { ::std::mem::uninitialized() };

        //Calculate the `NeighbourArrays`.
        for index in 0..Self::MINIMUM_TILES {
            res[index] = (
                neighbours[Tile::NEIGHBOURS],
                //Create a new `NeighbourArray` for this position.
                <&NeighbourArray>::try_from(&neighbours[..Tile::NEIGHBOURS])?.clone()
            );
            
            //Swap out the unused `Pos` for this `Tile`.
            unsafe { ::std::ptr::swap(&mut neighbours[index] as *mut _, &mut neighbours[Tile::NEIGHBOURS] as *mut _); }
        }
        
        //Return the result.
        Ok(res)
    }
    /// Attempts to create a default `TilePool`
    pub fn default() -> Result<Self, TilePoolError> {
        //Allocate enough spaces for the initial Tiles.
        let mut neighbours: [Pos; Self::MINIMUM_TILES] = unsafe { ::std::mem::uninitialized() };
        
        //Initialise unique tiles.
        for index in 0..Self::MINIMUM_TILES {
            neighbours[index] = Pos::new(0, index as i32)
        }
        
        let mut res = HashMap::with_capacity(Self::MINIMUM_TILES);
        
        //Return a new `TilePool`.
        for (pos, neighbours) in TilePool::connect_neighbours(neighbours)?.into_iter() {
            res.insert(pos.clone(), Tile::new_empty(neighbours.clone()));
        }

        Ok(TilePool(res))
    }
    /// Attempts to add `entity` to the `TilePool`.
    /// 
    /// Tries to insert `entity` into an existing `Tile` if it exists and is empty, then
    /// creates a new `Tile` for `entity`.  
    /// If a `Tile` at `pos` exists but is filled, `entity` is returned.
    /// 
    /// # Params
    /// 
    /// pos --- The `Pos` to insert `entity` at.
    /// entity --- The `Entity` to insert.
    pub fn add_to(&mut self, pos: &Pos, entity: Entity) -> Result<(), (Entity, TilePoolError)> {
        //Check if a `Tile` exists.
        if let Some(dest) = self.0.get_mut(pos) {
            //A `Tile` exists, check if it is empty.
            return if dest.inner == None {
                //The `Tile` is empty, place `entity` in it.
                Ok(dest.inner = Some(entity))
            } else {
                //The `Tile` is filled, return `entity`.
                Err((entity, TilePoolError::FilledTileError(pos.clone())))
            }
        }
        
        //No `Tile` exists, create a new `Tile` for `entity`.
        match self.calc_neighbours(NeighbourArray::default(), pos) {
            //The neighbours were calculated, insert into the `TilePool`.
            Ok(neighbours) => { self.0.insert(*pos, Tile::new(Some(entity), neighbours)); Ok(()) },
            //The neighbours could not be calculated, return the `Entity`.
            Err(e) => Err((entity, e))
        }
    }
    /// Attempts to extract an `Entity` from the `TilePool`.
    /// 
    /// If an `Entity` is found at `pos`, it is removed and returned.
    /// 
    /// # Params
    /// 
    /// pos --- The `Pos` to remove at.
    pub fn remove_from(&mut self, pos: &Pos) -> Option<Entity> {
        //Gets the `Tile` at `pos`.
        self.0.get_mut(pos)
        //Removes and returns the `Entity`.
        .and_then(|tile| tile.inner.take())
    }
    /// Ensures the passed `NeighbourArray` is optimised with its nearest neighbours in
    /// the `TilePool` for `pos`.
    /// 
    /// Running time of the function is lower the closer to the optimal solution `neighbours` is when passed in.
    /// Error indicates that 
    /// 
    /// # Params
    /// 
    /// neighbours --- The `NeighbourArray` to optimise.
    /// pos --- The `Pos` to optimise for.
    pub fn calc_neighbours(&self, mut neighbours: NeighbourArray, pos: &Pos) -> Result<NeighbourArray, TilePoolError> {
        //Calculates the neighbours recursively.
        fn inner_neighbours(pool: &TilePool, mut neighbours: NeighbourArray, first: usize, pos: &Pos) -> Result<NeighbourArray, TilePoolError> {
            //Loop though all the neighbours which need to be checked.
            for neigh_index in first..neighbours.len() {
                //Get the neighbour being pointed too currently.
                match pool.get(&neighbours[neigh_index]) {
                    //The neighbour could not be retrieved.
                    None => return Err(TilePoolError::RetrievalError(neighbours[neigh_index])),
                    //Loop through all the neighbours of `other`.
                    Some(other) => for &other_neigh in other.get_neighbours() {
                        //Check that the other neighbour is not pointing back to this tiles position.
                        if other_neigh != *pos {
                            //The index of the furthest neighbour.
                            let mut max_index = 0;

                            //Check that the first neighbour is not already the other neighbour.
                            if neighbours[max_index] == other_neigh {
                                //Loop through all neighbours.
                                for check_index in 1..neighbours.len() {
                                    //Check that the neighbour is not the other neighbour.
                                    if neighbours[check_index] == other_neigh {
                                        //The other neighbour is already being tracked, skip it.
                                        max_index = neighbours.len(); break
                                    //Check if the other neighbour is closer than the current neighbour.
                                    } else if neighbours[check_index].dist2_from(pos) > neighbours[max_index].dist2_from(pos) {
                                        max_index = check_index
                                    }
                                }
                                
                                //Check that a maximum was found successfully.
                                if max_index < neighbours.len() {
                                    //A maximum was found successfully.
                                    //Replace the furthest neighbour with this one.
                                    neighbours[max_index] = other_neigh;
                                    //Check that the index the neighbour was inserted at does not require we restart from there.
                                    if max_index < neigh_index {
                                        //The new neighbour will not be checked, restart the checking from there.
                                        return inner_neighbours(pool, neighbours, first, pos)
                                    }
                                }
                            }
                        }
                    }
                }
            }

            //This is the optimal solution.
            Ok(neighbours)
        }

        //Check that there are enough `Tiles` in the `TilePool`.
        if self.len() < Self::MINIMUM_TILES { return Err(TilePoolError::SizeError(self.len())) }

        if let Some(tile) = self.get(&pos) { return Ok(tile.get_neighbours().clone()) }

        //Check that the `NeighbourArray` only contains valid neighbours.
        for (pos, neighbour) in neighbours.iter_mut()
            //Filter only the invalid neighbours.
            .filter(|pos| self.get(pos).is_none())
            //Get neighbours from the `TilePool` as replacements.
            .zip(self.0.iter().map(|(pos, _)| pos)) {
            //Replace the neighbour.
            *pos = *neighbour
        }

        //Calculate the neighbours.
        inner_neighbours(self, neighbours, 0, pos)
    }
}

impl Deref for TilePool {
    type Target = HashMap<Pos, Tile>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use entities::Unit;

    #[test]
    fn test_tilepool() {
        let mut pool = TilePool::default()
            .expect("`TilePool::default` failed.");
        let pos = Pos::new(3, 4);
        let entity = Box::new(Unit).into();

        if let Err((_, e)) = pool.add_to(&pos, entity) {
            panic!("`TilePool::add_to` failed: {}", e)
        }

        if let None = pool.remove_from(&pos) {
            panic!("`TilePool::remove_from` failed.")
        }
    }
}
