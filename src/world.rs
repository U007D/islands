mod terrain;
#[cfg(test)]
mod unit_tests;

use std::{
    fmt::Debug,
    slice::{Iter, IterMut},
};

use bool_ext::BoolExt;
pub use terrain::Terrain;

use crate::error::world_map::{Error, Result};

/// Represents a non-empty, contiguous, rectangular (all rows are the same length) 2D list of items.
/// Despite the internal representation being a 1D `Box`ed (heap-allocated) slice,
/// for convenience and to help ensure correctness, [`World`] presents a 2D
/// (row, column) interface to the user.
///
/// # Returns:
/// `Err(Error::NoData(String))`: when `rows == 0` or `cols == 0`.
/// `Err(Error::TooManyElements(rows, cols))`: when `rows * cols > isize::MAX.into()`.
/// `Ok(Self)`: otherwise.
///
/// # Panics
/// Does not panic, but `Vec` allocation may panic (until Rust stabilizes fallible collections).
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct World {
    list: Box<[Terrain]>,
    cols: usize,
    rows: usize,
}

impl World {
    /// Constructor
    pub fn new(terrain: Terrain, rows: usize, cols: usize) -> Result<Self> {
        // `WorldMap::rows * WorldMap::cols` cannot overflow, so index integer arithmetic is safe.
        #[allow(clippy::integer_arithmetic)]
        is_isized_usize(rows, cols).map_or(Err(Error::TooManyElements(rows, cols)), || {
            (vec![terrain; rows * cols], rows, cols).try_into()
        })
    }
}

impl World {
    /// Returns the number of columns (width) of the 2D list.
    #[inline]
    #[must_use]
    pub const fn cols(&self) -> usize {
        self.cols
    }

    /// Returns an optional reference to the value at `row`, `col`, if the coordinates exist.
    #[inline]
    #[must_use]
    pub fn get(&self, row: usize, col: usize) -> Option<&Terrain> {
        row.checked_mul(self.cols())
            .and_then(|el_row| el_row.checked_add(col).and_then(|idx| self.list.get(idx)))
    }

    /// Consuming `Vec` converter.
    #[must_use]
    pub fn into_vec(self) -> Vec<Terrain> {
        self.list.into_vec()
    }

    /// Predicate indicating whether or not the `Terrain` at the given location is land or water.
    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// Predicate indicating whether or not the `Terrain` at the given location is land or water.
    #[must_use]
    #[inline]
    // `WorldMap::rows * WorldMap::cols` cannot overflow, so index integer arithmetic is safe.
    #[allow(clippy::integer_arithmetic)]
    pub fn is_land(&self, row: usize, col: usize) -> Option<bool> {
        self.list.get(row * col + col).map(|&terrain| terrain == Terrain::Land)
    }

    /// Immutable iterator constructor.
    pub fn iter(&self) -> Iter<'_, <Self as IntoIterator>::Item> {
        self.list.iter()
    }

    /// Mutable iterator constructor.
    pub fn iter_mut(&mut self) -> IterMut<'_, <Self as IntoIterator>::Item> {
        self.list.iter_mut()
    }

    /// Sets the value at coordinates `row`, `col` and returns optional `&mut Self` if the
    /// coordinates exist.
    #[inline]
    #[must_use]
    pub fn set(&mut self, row: usize, col: usize, value: Terrain) -> Option<&mut Self> {
        row.checked_mul(self.cols()).and_then(|el_row| {
            el_row.checked_add(col).map(|idx| {
                self.list.get_mut(idx).map(|entry| *entry = value);
                self
            })
        })
    }

    /// Returns the number of rows (height) of the 2D list.
    #[inline]
    #[must_use]
    pub const fn rows(&self) -> usize {
        self.rows
    }
}

/// Consuming iterator constructor.
impl IntoIterator for World {
    type IntoIter = <Vec<Terrain> as IntoIterator>::IntoIter;
    type Item = Terrain;

    fn into_iter(self) -> Self::IntoIter {
        self.list.into_vec().into_iter()
    }
}

/// Converting Constructor
impl TryFrom<(Vec<Terrain>, usize, usize)> for World {
    type Error = Error;

    fn try_from((vec, rows, cols): (Vec<Terrain>, usize, usize)) -> Result<Self, Self::Error> {
        is_isized_usize(rows, cols).err(Error::TooManyElements(rows, cols))?;
        let world = Self {
            list: vec.into_boxed_slice(),
            rows,
            cols,
        };
        Ok(world)
    }
}

/// DRY implementation ensures that the product of `row` and `col` does not exceed `isize::MAX`
/// (note: the limit is not `usize::MAX`).  This limitation is a
/// [Rust limitation on collection sizes driven by LLVM](https://doc.rust-lang.org/stable/reference/types/numeric.html#machine-dependent-integer-types).
fn is_isized_usize(row: usize, col: usize) -> bool {
    row.checked_mul(col).map_or(false, |element_count| isize::try_from(element_count).is_ok())
}
