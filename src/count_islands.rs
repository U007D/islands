use crate::{VisitedWorld, World};

#[cfg(test)]
mod unit_tests;

/// Given a reference to a [`World`] instance, compute the number of islands in that `World`.  An
/// island is defined as vertically or horizontally contiguous land mass.  World edges do not wrap.
#[must_use]
pub fn count_islands(world_ref: &World) -> usize {
    let mut visited = VisitedWorld::from(world_ref);
    (0..world_ref.rows())
        .map(|row| {
            (0..world_ref.cols())
                .filter(|&col| {
                    visited
                        .is_unvisited_land(row, col)
                        .then(|| visited.visit_contiguous_land(row, col))
                        .is_some()
                })
                .count()
        })
        .sum()
}
