use std::ops::{Deref, DerefMut};
use {BoundingBox, Bitmap};

/// A font glyph.
#[derive(Debug)]
pub struct Glyph {
	name:      String,
	codepoint: char,

	scalable_width: (u32, u32),
	device_width:   (u32, u32),

	bounds: Option<BoundingBox>,
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

			bounds: Default::default(),
			map:    Default::default(),
		}
	}

	/// Gets the name.
	pub fn name(&self) -> &str {
		&self.name
	}

	/// Sets the name.
	pub fn set_name<T: Into<String>>(&mut self, name: T) {
		self.name = name.into();
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
	pub fn bounds(&self) -> Option<&BoundingBox> {
		self.bounds.as_ref()
	}

	/// Sets the bounds.
	pub fn set_bounds(&mut self, bounds: Option<BoundingBox>) {
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

	/// Create an iterator over the pixels which will yield `((x, y), value)`.
	pub fn pixels(&self) -> PixelIter {
		PixelIter {
			x: 0,
			y: 0,

			map: &self.map,
		}
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

pub struct PixelIter<'a> {
	x: u32,
	y: u32,

	map: &'a Bitmap,
}

impl<'a> Iterator for PixelIter<'a> {
	type Item = ((u32, u32), bool);

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		if self.x >= self.map.width() {
			self.x  = 0;
			self.y += 1;
		}

		if self.y >= self.map.height() {
			return None;
		}

		let x = self.x;
		let y = self.y;

		self.x += 1;

		Some(((x, y), self.map.get(x, y)))
	}
}
