#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct BoundingBox {
	width:  u32,
	height: u32,

	x: i32,
	y: i32,
}

impl BoundingBox {
	pub fn empty() -> Self {
		BoundingBox::new(0, 0, 0, 0)
	}

	pub fn new(width: u32, height: u32, x: i32, y: i32) -> Self {
		BoundingBox {
			width:  width,
			height: height,

			x: x,
			y: y,
		}
	}

	pub fn width(&self) -> u32 {
		self.width
	}

	pub fn height(&self) -> u32 {
		self.height
	}

	pub fn x(&self) -> i32 {
		self.x
	}

	pub fn y(&self) -> i32 {
		self.y
	}
}
