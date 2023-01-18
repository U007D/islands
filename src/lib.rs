// Safety-critical application lints
#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used
)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
#![allow(
    clippy::equatable_if_let,
    clippy::implicit_return,
    clippy::iter_nth_zero,
    clippy::option_map_unit_fn,
    clippy::match_bool,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::wildcard_imports
)]
// To use the `unsafe` keyword, do not remove the `unsafe_code` attribute entirely.
// Instead, change it to `#![allow(unsafe_code)]` or preferably `#![deny(unsafe_code)]` + opt-in
// with local `#[allow(unsafe_code)]`'s on a case-by-case basis, if practical.
#![forbid(unsafe_code)]
#![forbid(bare_trait_objects)]
// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing
// license files and more
// #![allow(clippy::blanket_clippy_restriction_lints)]
// #![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
// #![allow(clippy::implicit_return)]

pub mod args;
pub mod error;
mod non_empty_rect_list_2d;
pub mod shared_consts;
#[cfg(test)]
mod unit_tests;
mod visited_world;
mod world;

pub use error::{Error, Result};
pub use non_empty_rect_list_2d::NonEmptyRectList2D;
pub use visited_world::VisitedWorld;
pub use world::World;

#[must_use]
pub fn count_islands(world_ref: &World) -> usize {
    let mut visited = VisitedWorld::from(world_ref);
    (0..visited.rows())
        .map(|row| {
            (0..visited.cols())
                .filter(|&col| {
                    visited
                        .is_unvisited_land(row, col)
                        .then_some(|| visited.visit_contiguous_land(row, col))
                        .is_some()
                })
                .count()
        })
        .sum()
}
