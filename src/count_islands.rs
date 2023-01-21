use std::collections::HashSet;

use crate::World;

#[cfg(test)]
mod unit_tests;

/// Given a reference to a [`World`] instance, compute the number of islands in that `World`.  An
/// island is defined as vertically or horizontally contiguous land mass.  World edges do not wrap.
#[must_use]
pub fn count_islands(world: &World) -> usize {
    let mut visited = HashSet::new();
    (0..world.rows()).map(|row| {
                         (0..world.cols())
                // Retain current element if it is land and not yet visited.  Mark as visited.
                .filter(|&col| {
                    world.is_land(row, col).unwrap_or(false) && visited.insert((row, col))
                })
                .count()
                     })
                     .sum()
}
