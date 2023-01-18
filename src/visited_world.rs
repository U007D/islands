use std::ops::Not;

use crate::{NonEmptyRectList2D, World};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct VisitedWorld<'world> {
    world: &'world World,
    visited: NonEmptyRectList2D<bool>,
}

impl<'world> VisitedWorld<'world> {
    #[inline]
    #[must_use]
    pub fn cols(&self) -> usize {
        self.world.cols()
    }

    #[inline]
    #[must_use]
    pub fn is_unvisited_land(&self, row: usize, col: usize) -> bool {
        self.world.is_land(row, col).unwrap_or(false)
            && self.visited.get(row, col).map_or(false, |&visited| visited.not())
    }

    #[inline]
    #[must_use]
    pub fn rows(&self) -> usize {
        self.world.rows()
    }

    #[inline]
    pub fn set_visited(&mut self, row: usize, col: usize) -> &mut Self {
        self.visited.set(row, col, true).unwrap_or_else(|| unreachable!());
        self
    }

    pub fn visit_contiguous_land(&mut self, row: usize, col: usize) {
        self.visit_contiguous_land_inner(row, col, 0);
    }

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

impl<'world> From<&'world World> for VisitedWorld<'world> {
    fn from(world: &'world World) -> Self {
        Self {
            visited: NonEmptyRectList2D::new(false, world.rows(), world.cols())
                .unwrap_or_else(|err| unreachable!("{err:}")),
            world,
        }
    }
}
