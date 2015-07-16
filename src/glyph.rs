use std::ops::{Deref, DerefMut};
use {BoundingBox, Bitmap};

/// A font glyph.
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
	/// Creates an empty glyph.
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

	/// Gets the name.
	pub fn name(&self) -> &str {
		&self.name
	}

	/// Sets the name.
	pub fn set_name(&mut self, name: String) {
		self.name = name;
	}

	/// Gets the codepoint.
	pub fn codepoint(&self) -> char {
		self.codepoint
	}

	/// Sets the codepoint.
	pub fn set_codepoint(&mut self, codepoint: char) {
		self.codepoint = codepoint;
	}

	/// Gets the scalable width.
	pub fn scalable_width(&self) -> (u32, u32) {
		self.scalable_width
	}

	/// Sets the scalable width.
	pub fn set_scalable_width(&mut self, x: u32, y: u32) {
		self.scalable_width = (x, y);
	}

	/// Gets the device width.
	pub fn device_width(&self) -> (u32, u32) {
		self.device_width
	}

	/// Sets the device width.
	pub fn set_device_width(&mut self, x: u32, y: u32) {
		self.device_width = (x, y);
	}

	/// Gets the bounds.
	pub fn bounds(&self) -> &BoundingBox {
		&self.bounds
	}

	/// Sets the bounds.
	pub fn set_bounds(&mut self, bounds: BoundingBox) {
		self.bounds = bounds;
	}

	/// Gets the bitmap.
	pub fn map(&self) -> &Bitmap {
		&self.map
	}

	/// Sets the bitmap.
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
