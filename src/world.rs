mod terrain;
#[cfg(test)]
mod unit_tests;

use std::{
    fmt::Debug,
    num::NonZeroUsize,
};

pub use crate::{error::non_empty_rect_list_2d::Result, NonEmptyRectList2D};
pub use terrain::Terrain;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct World {
    list: NonEmptyRectList2D<Terrain>,
    rows: usize,
    cols: usize,
}

impl World {
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
    #[must_use]
    #[inline]
    #[allow(clippy::missing_const_for_fn)]
    pub fn cols(&self) -> usize {
        self.cols
    }

    #[must_use]
    #[inline]
    #[allow(clippy::missing_const_for_fn)]
    pub fn rows(&self) -> usize {
        self.rows
    }

    #[must_use]
    #[inline]
    pub fn is_land(&self, row: usize, col: usize) -> Option<bool> {
        self.list.get(row, col).map(|&terrain| terrain == Terrain::Land)
    }
}
