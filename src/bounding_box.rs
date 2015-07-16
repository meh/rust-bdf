/// The bounds of a glyph.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct BoundingBox {
	///
	pub width: u32,

	///
	pub height: u32,

	///
	pub x: i32,

	///
	pub y: i32,
}

impl Default for BoundingBox {
	fn default() -> Self {
		BoundingBox {
			width:  0,
			height: 0,

			x: 0,
			y: 0,
		}
	}
}
