
use std::ops::{Deref, DerefMut};
use position::*;
use entities::Entity;

/// An Array of neighbours for a `Tile`.
pub type NeighbourArray = [Pos; Tile::NEIGHBOURS];

/// A `Tile` contains an optional `Entity` and tracks its nearest neighbours.
pub struct Tile {
    /// The optional `Entity`.
    pub inner: Option<Entity>,
    /// The nearest neighbours of the `Tile`.
    neighbours: NeighbourArray,
}

impl Tile {
    /// The number of neighbours a `Tile` tracks.
    pub const NEIGHBOURS: usize = 5;

    /// Creates a new `Tile` from parts.
    pub fn new(inner: Option<Entity>, neighbours: NeighbourArray) -> Self { Self { inner, neighbours } }
    /// Creates a new `Tile` without an `Entity`.
    pub fn new_empty(neighbours: NeighbourArray) -> Self { Self::new(None, neighbours) }
    pub fn get_neighbours(&self) -> &[Pos; Self::NEIGHBOURS] { &self.neighbours }
}

impl Deref for Tile {
    type Target = Option<Entity>;

    fn deref(&self) -> &Self::Target { &self.inner }
}

impl DerefMut for Tile {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
}
