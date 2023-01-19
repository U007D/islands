mod terrain;
#[cfg(test)]
mod unit_tests;

use std::{
    fmt::Debug,
    num::NonZeroUsize,
};

pub use crate::{error::non_empty_rect_list_2d::Result, NonEmptyRectList2D};
pub use terrain::Terrain;

/// Provides the [`Terrain`] type ([`Terrain::Water`] or [`Terrain::Land`]) for each location in the
/// world.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct World {
    list: NonEmptyRectList2D<Terrain>,
    rows: usize,
    cols: usize,
}

impl World {
    /// Constructor
    ///
    /// # Returns
    /// * `Err(error::non_empty_rec_list_2d::Error)` if total element count is 0 or `> isize::MAX`
    /// `isize::MAX` is a Rust `std` constraint driven directly by LLVM.
    ///
    /// # Panics
    /// Does not panic, but underlying `Vec` allocation may panic (until Rust stabilizes fallible
    /// collections).
    pub fn new<TIntoNonZeroUsize>(
        terrain: Terrain,
        rows: TIntoNonZeroUsize,
        cols: TIntoNonZeroUsize,
    ) -> Result<Self>
    where
        TIntoNonZeroUsize: TryInto<NonZeroUsize>,
        <TIntoNonZeroUsize as TryInto<NonZeroUsize>>::Error: Debug,
    {
        let list = NonEmptyRectList2D::new(terrain, rows, cols)?;
        let world = Self {
            rows: list.rows(),
            cols: list.cols(),
            list,
        };
        Ok(world)
    }
    /// Returns the number of columns (width) of the 2D `World` list.
    #[must_use]
    #[inline]
    #[allow(clippy::missing_const_for_fn)]
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns the number of rows (height) of the 2D `World` list.
    #[must_use]
    #[inline]
    #[allow(clippy::missing_const_for_fn)]
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Predicate indicating whether or not the `Terrain` at the given location is land or water.
    #[must_use]
    #[inline]
    pub fn is_land(&self, row: usize, col: usize) -> Option<bool> {
        self.list.get(row, col).map(|&terrain| terrain == Terrain::Land)
    }
}
