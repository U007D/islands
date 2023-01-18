#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Terrain {
    #[default]
    Water,
    Land,
}
