/// The bounds of a glyph.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct BoundingBox {
	width:  u32,
	height: u32,

	x: i32,
	y: i32,
}

impl BoundingBox {
	/// Creates an empty bounding box.
	pub fn empty() -> Self {
		BoundingBox::new(0, 0, 0, 0)
	}

	/// Creates a new bounding box with the given bounds.
	pub fn new(width: u32, height: u32, x: i32, y: i32) -> Self {
		BoundingBox {
			width:  width,
			height: height,

			x: x,
			y: y,
		}
	}

	/// Gets the width.
	pub fn width(&self) -> u32 {
		self.width
	}

	/// Gets the height.
	pub fn height(&self) -> u32 {
		self.height
	}

	/// Gets the x.
	pub fn x(&self) -> i32 {
		self.x
	}

	/// Gets the y.
	pub fn y(&self) -> i32 {
		self.y
	}
}
