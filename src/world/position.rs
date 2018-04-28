
use std::ops::*;

/// A `Pos` is a coordinate in space.
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Pos {
    /// The `x` coord.
    pub x: i32,
    /// The `y` coord.
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self { Self { x, y } }
    /// Calculate the dot product of the two `Positons`.
    /// 
    /// # Params
    /// 
    /// a --- The first `Pos` in the product.
    /// b --- The second `Pos` in the product.
    pub fn dot(a: &Self, b: &Self) -> u32 {
        (a.x * a.x) as u32 + (b.y * b.y) as u32
    }
    /// Calculates the magnituid squared of this `Pos`.
    pub fn mag2(&self) -> u32 {
        Self::dot(self, self)
    }
    /// Calculates the distance squared between this `Pos` and `other`.
    /// 
    /// # Params
    /// 
    /// other --- The `Pos` to calculate the distance from.
    pub fn dist2_from(&self, other: &Self) -> u32 {
        (*self - *other).mag2()
    }
}

impl Default for Pos {
    fn default() -> Self { Pos::new(0, 0) }
}

impl Neg for Pos {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.x = -self.x;
        self.y = -self.y;
        self
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output { self += rhs; self }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output { self -= rhs; self }
}

impl SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

/// A type is a `Positionable` if it can return a position and it's position can be set.
pub trait Positionable: Sized {
    /// Return the position of this `Positionable`.
    fn get_position(&self) -> &Pos;
    /// Sets the position of this `Positionable`.
    fn set_position(self, pos: &Pos) -> Self;
    /// Translates the `Positionable`.
    fn translate(self, mut trans: Pos) -> Self {
        trans += *self.get_position();
        self.set_position(&trans)
    }
}

impl<T: AsMut<Pos> + AsRef<Pos>> Positionable for T {
    fn get_position(&self) -> &Pos { self.as_ref() }
    fn set_position(mut self, pos: &Pos) -> Self { *self.as_mut() = *pos; self }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos() {
        let pos = Pos::new(1, 2);

        assert_eq!(Pos::dot(&pos, &pos), 5, "`Pos::dot` failed.");
        assert_eq!(pos.mag2(), Pos::dot(&pos, &pos), "`Pos::mag2` failed.");
        assert_eq!(pos.dist2_from(&Pos::new(1, 1)), 1, "`Pos::dist2_from` failed.");
    }
}
