/// The direction of the glyph.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Direction {
    /// Default direction, typically lef-to-right.
    Default,

    /// Alternate direction, typically right-to-left.
    Alternate,

    /// Both directions.
    Both,
}

impl Default for Direction {
    #[inline]
    fn default() -> Self {
        Direction::Default
    }
}
