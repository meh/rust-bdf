use std::ops::{Deref, DerefMut};
use {BoundingBox, Bitmap};

#[derive(Debug)]
pub struct Glyph {
	name:      String,
	codepoint: char,

	scalable_width: (u32, u32),
	device_width:   (u32, u32),

	bounds: BoundingBox,
	map:    Bitmap,
}

impl Glyph {
	pub fn empty() -> Self {
		Glyph {
			name:      "".to_owned(),
			codepoint: '\u{0}',

			scalable_width: (0, 0),
			device_width:   (0, 0),

			bounds: BoundingBox::empty(),
			map:    Bitmap::empty(),
		}
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn set_name(&mut self, name: String) {
		self.name = name;
	}

	pub fn codepoint(&self) -> char {
		self.codepoint
	}

	pub fn set_codepoint(&mut self, codepoint: char) {
		self.codepoint = codepoint;
	}

	pub fn scalable_width(&self) -> (u32, u32) {
		self.scalable_width
	}

	pub fn set_scalable_width(&mut self, x: u32, y: u32) {
		self.scalable_width = (x, y);
	}

	pub fn device_width(&self) -> (u32, u32) {
		self.device_width
	}

	pub fn set_device_width(&mut self, x: u32, y: u32) {
		self.device_width = (x, y);
	}

	pub fn bounds(&self) -> &BoundingBox {
		&self.bounds
	}

	pub fn set_bounds(&mut self, bounds: BoundingBox) {
		self.bounds = bounds;
	}

	pub fn map(&self) -> &Bitmap {
		&self.map
	}

	pub fn set_map(&mut self, map: Bitmap) {
		self.map = map;
	}
}

impl Deref for Glyph {
	type Target = Bitmap;

	fn deref(&self) -> &Bitmap {
		&self.map
	}
}

impl DerefMut for Glyph {
	fn deref_mut(&mut self) -> &mut Bitmap {
		&mut self.map
	}
}
