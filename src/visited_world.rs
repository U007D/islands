#[cfg(test)]
mod unit_tests;

use std::{
    ops::Not,
    slice::{Iter, IterMut},
};

use crate::{NonEmptyRectList2D, World};

/// [`VisitedWorld`] wraps a [`World`] reference, and creates an internal [`NonEmptyRectList2D`] of
/// identical dimensions to the `World` reference to track whether each location of the world has
/// been "visited" or not.  `VisitedWorld` starts out with every location "unvisited" (false).
/// When determining what is an island, to do so efficiently, we mark where we have examined
/// previously as "visited" (true) so that we can solve in O(n) time instead of O(n^2).
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VisitedWorld<'world> {
    world: &'world World,
    visited_list: NonEmptyRectList2D<bool>,
}

impl<'world> VisitedWorld<'world> {
    /// Returns the number of columns (width) of the 2D Visited list.
    #[inline]
    #[must_use]
    pub fn cols(&self) -> usize {
        self.world.cols()
    }

    /// Predicate indicating whether the location at the specified coordinate is both
    /// [`Terrain::Land`] and unvisited.  Out-of-bounds coordinates always return `false`.
    #[inline]
    #[must_use]
    pub fn is_unvisited_land(&self, row: usize, col: usize) -> bool {
        self.world.is_land(row, col).unwrap_or(false)
            && self.visited_list.get(row, col).map_or(false, |&visited| visited.not())
    }

    /// Immutable iterator constructor.
    pub fn iter(&self) -> Iter<'_, <Self as IntoIterator>::Item> {
        self.visited_list.iter()
    }

    /// Mutable iterator constructor.
    pub fn iter_mut(&mut self) -> IterMut<'_, <Self as IntoIterator>::Item> {
        self.visited_list.iter_mut()
    }

    /// Returns the number of rows (height) of the 2D Visited list.
    #[inline]
    #[must_use]
    pub fn rows(&self) -> usize {
        self.world.rows()
    }

    /// Unconditionally mark the specified location as having been visited.  Writing to
    /// out-of-bounds co-ordinates is treated as a no-op.
    #[inline]
    pub fn set_visited(&mut self, row: usize, col: usize, visited: bool) -> &mut Self {
        let dbg_success = self.visited_list.set(row, col, visited).is_some();
        // Detect out-of-bounds write attempts in debug builds
        debug_assert!(
            dbg_success,
            "`VisitedWorld::set_visited()` called with OOB coords: ({row}, {col})"
        );
        self
    }

    /// Recursively explore unvisited land that is vertically or horizontally adjacent to the given
    /// coordinate, to world's edge or until water is encountered, marking each location as visited
    /// as we go.  The state at the starting search coordinate will be left unchanged.
    pub fn visit_contiguous_land(&mut self, row: usize, col: usize) -> &mut Self {
        self.visit_contiguous_land_inner(row, col, 0);
        self
    }

    /// Track the depth of recursion.  With this depth information the algorithm can differentiate
    /// between the first call and all subsequent recursive calls to restore the initial
    /// state of the starting search coordinate.
    fn visit_contiguous_land_inner(&mut self, row: usize, col: usize, depth: usize) {
        self.is_unvisited_land(row, col).then(|| {
            self.set_visited(row, col, true);
            let deeper = depth.saturating_add(1);
            (row.checked_sub(1))
                .map(|prev_row| self.visit_contiguous_land_inner(prev_row, col, deeper));
            (row.checked_add(1))
                .map(|next_row| self.visit_contiguous_land_inner(next_row, col, deeper));
            (col.checked_sub(1))
                .map(|prev_col| self.visit_contiguous_land_inner(row, prev_col, deeper));
            (col.checked_add(1))
                .map(|next_col| self.visit_contiguous_land_inner(row, next_col, deeper));
            (depth == 0).then(|| self.set_visited(row, col, false));
        });
    }
}

/// Constructor.  Creates a [`VisitedWorld`] of identical dimensions to the [`World`] reference
/// passed in (i.e. does not take ownership of the `World`) so that each `World` location has a
/// corresponding (mutable) visited status.
///
/// # Panics
/// Does not panic, but underlying `Vec` allocation may panic (until Rust stabilizes fallible
/// collections).
impl<'world> From<&'world World> for VisitedWorld<'world> {
    fn from(world: &'world World) -> Self {
        Self {
            visited_list: NonEmptyRectList2D::new(false, world.rows(), world.cols())
                .unwrap_or_else(|err| unreachable!("{err:}")),
            world,
        }
    }
}

/// Consuming iterator constructor.
impl<'world> IntoIterator for VisitedWorld<'world> {
    type Item = bool;
    type IntoIter = <Vec<bool> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.visited_list.into_vec().into_iter()
    }
}
