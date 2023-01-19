use std::ops::Not;

use crate::{NonEmptyRectList2D, World};

/// [`VisitedWorld`] wraps a [`World`] reference, and creates an internal [`NonEmptyRectList2D`] of
/// identical dimensions to the `World` reference to track whether each location of the world has
/// been "visited" or not.  `VisitedWorld` starts out with every location "unvisited" (false).
/// When determining what is an island, to do so efficiently, we mark where we have examined
/// previously as "visited" (true) so that we can solve in O(n) time instead of O(n^2).
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VisitedWorld<'world> {
    world: &'world World,
    visited: NonEmptyRectList2D<bool>,
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
            && self.visited.get(row, col).map_or(false, |&visited| visited.not())
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
    pub fn set_visited(&mut self, row: usize, col: usize) -> &mut Self {
        let dbg_success = self.visited.set(row, col, true).is_some();
        // Detect out-of-bounds write attempts in debug builds
        debug_assert!(
            dbg_success,
            format!("`VisitedWorld::set_visited()` called with OOB coords: ({row}, {col})")
        );
        self
    }

    /// Recursively explore unvisited land that is vertically or horizontally adjacent to the given
    /// coordinate, to world's edge or until water is encountered, marking each location as visited
    /// as we go.  The initial starting coordinate is *not* marked as visited.
    pub fn visit_contiguous_land(&mut self, row: usize, col: usize) {
        self.visit_contiguous_land_inner(row, col, 0);
    }

    /// Track the depth of recursion.  With this depth information the algorithm can differentiate
    /// between the first call and all subsequent recursive calls to avoid marking the initial
    /// coordinate as visited.
    fn visit_contiguous_land_inner(&mut self, row: usize, col: usize, depth: usize) {
        self.is_unvisited_land(row, col).then(|| {
            (depth > 0).then(|| self.set_visited(row, col));

            let deeper = depth.saturating_add(1);
            (row.checked_sub(1))
                .map(|prev_row| self.visit_contiguous_land_inner(prev_row, col, deeper));
            (row.checked_add(1))
                .map(|next_row| self.visit_contiguous_land_inner(next_row, col, deeper));
            (col.checked_sub(1))
                .map(|prev_col| self.visit_contiguous_land_inner(row, prev_col, deeper));
            (col.checked_add(1))
                .map(|next_col| self.visit_contiguous_land_inner(row, next_col, deeper));
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
            visited: NonEmptyRectList2D::new(false, world.rows(), world.cols())
                .unwrap_or_else(|err| unreachable!("{err:}")),
            world,
        }
    }
}
