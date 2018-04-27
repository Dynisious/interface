
use std::fmt::{Display, Formatter, Result};
use position::Pos;

/// Errors which can occour while using a `TilePool`.
#[derive(Debug)]
pub enum TilePoolError {
    /// A `NeighbourArray` could not be created because the `TilePool` has too few `Tiles`.
    SizeError(usize),
    /// There was an error while retrieving a `Tile` from a `TilePool`.
    RetrievalError(Pos),
    /// An insertion was attempted on a filled `Tile`.
    FilledTileError(Pos),
    /// A neighbour was repeated in a `NeighbourArray`.
    RepeatedNeighbour(Pos),
    /// There was an error converting a `Slice` into a `NeighbourArray`.
    ConversionError,
}

impl Display for TilePoolError {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        match self {
            TilePoolError::SizeError(size) =>
                write!(fmt, "The `TilePool` passed had too few tiles ({} need {}).", size, super::TilePool::MINIMUM_TILES),
            TilePoolError::RetrievalError(pos) =>
                write!(fmt, "Failed to retrieve a `Tile` at {:?}; the `TilePool` is inconsistent.", pos),
            TilePoolError::FilledTileError(pos) =>
                write!(fmt, "Failed to insert into the `Tile` at {:?}; the `Tile` is already filled.", pos),
            TilePoolError::RepeatedNeighbour(pos) =>
                write!(fmt, "The neighbour {:?} was repeated in the `NeighbourArray`.", pos),
            TilePoolError::ConversionError =>
                write!(fmt, "There was an error converting a `Slice` into a `NeighbourArray`."),
        }
    }
}

impl From<::core::array::TryFromSliceError> for TilePoolError {
    fn from(_: ::core::array::TryFromSliceError) -> Self {
        TilePoolError::ConversionError
    }
}
