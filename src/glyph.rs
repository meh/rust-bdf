use std::ops::{Deref, DerefMut};
use {BoundingBox, Bitmap, Direction};

/// A font glyph.
#[derive(Debug)]
pub struct Glyph {
	name:      Option<String>,
	codepoint: Option<char>,

	direction: Direction,

	scalable_width: Option<(u32, u32)>,
	device_width:   Option<(u32, u32)>,

	alternate_scalable_width: Option<(u32, u32)>,
	alternate_device_width:   Option<(u32, u32)>,

	vector: Option<(u32, u32)>,

	bounds: Option<BoundingBox>,
	map:    Bitmap,
}

impl Default for Glyph {
	fn default() -> Self {
		Glyph {
			name:      None,
			codepoint: None,

			direction: Default::default(),

			scalable_width: None,
			device_width:   None,

			alternate_scalable_width: None,
			alternate_device_width:   None,

			vector: None,

			bounds: Default::default(),
			map:    Default::default(),
		}
	}
}

impl Glyph {
	/// Creates a new glyph with the given name and codepoint.
	pub fn new<T: Into<String>>(name: T, codepoint: char) -> Self {
		Glyph {
			name:      Some(name.into()),
			codepoint: Some(codepoint),

			.. Default::default()
		}
	}

	/// Validates the definition.
	pub fn validate(&self) -> bool {
		if self.name.is_none() {
			return false;
		}

		if self.codepoint.is_none() {
			return false;
		}

		if self.bounds.is_none() {
			return false;
		}

		if self.direction == Direction::Default {
			if self.alternate_scalable_width.is_some() {
				return false;
			}

			if self.alternate_device_width.is_some() {
				return false;
			}
		}
		else {
			if self.alternate_scalable_width.is_none() {
				return false;
			}

			if self.alternate_device_width.is_none() {
				return false;
			}
		}

		true
	}

	/// Gets the name.
	pub fn name(&self) -> &str {
		&self.name.as_ref().unwrap().as_ref()
	}

	/// Sets the name.
	pub fn set_name<T: Into<String>>(&mut self, name: T) {
		self.name = Some(name.into());
	}

	/// Gets the codepoint.
	pub fn codepoint(&self) -> char {
		self.codepoint.unwrap()
	}

	/// Sets the codepoint.
	pub fn set_codepoint(&mut self, codepoint: char) {
		self.codepoint = Some(codepoint);
	}

	/// Gets the direction.
	pub fn direction(&self) -> Direction {
		self.direction
	}

	/// Sets the direction.
	pub fn set_direction(&mut self, direction: Direction) {
		self.direction = direction;
	}

	/// Gets the scalable width.
	pub fn scalable_width(&self) -> Option<&(u32, u32)> {
		self.scalable_width.as_ref()
	}

	/// Sets the scalable width.
	pub fn set_scalable_width(&mut self, value: Option<(u32, u32)>) {
		self.scalable_width = value;
	}

	/// Gets the device width.
	pub fn device_width(&self) -> Option<&(u32, u32)> {
		self.device_width.as_ref()
	}

	/// Sets the device width.
	pub fn set_device_width(&mut self, value: Option<(u32, u32)>) {
		self.device_width = value;
	}

	/// Gets the alternate scalable width.
	pub fn alternate_scalable_width(&self) -> Option<&(u32, u32)> {
		self.alternate_scalable_width.as_ref()
	}

	/// Sets the alternate scalable width.
	pub fn set_alternate_scalable_width(&mut self, value: Option<(u32, u32)>) {
		self.alternate_scalable_width = value;
	}

	/// Gets the alternate device width.
	pub fn alternate_device_width(&self) -> Option<&(u32, u32)> {
		self.alternate_device_width.as_ref()
	}

	/// Sets the alternate device width.
	pub fn set_alternate_device_width(&mut self, value: Option<(u32, u32)>) {
		self.alternate_device_width = value;
	}

	/// Gets the offset vector.
	pub fn vector(&self) -> Option<&(u32, u32)> {
		self.vector.as_ref()
	}

	/// Sets the offset vector.
	pub fn set_vector(&mut self, value: Option<(u32, u32)>) {
		self.vector = value;
	}

	/// Gets the bounds.
	pub fn bounds(&self) -> &BoundingBox {
		self.bounds.as_ref().unwrap()
	}

	/// Sets the bounds.
	pub fn set_bounds(&mut self, bounds: BoundingBox) {
		self.bounds = Some(bounds);
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

	fn size_hint(&self) -> (usize, Option<usize>) {
		let width   = self.map.width();
		let height  = self.map.height();
		let current = (self.y * height) + self.x;

		(current as usize, Some(((width * height) - current) as usize))
	}
}

impl<'a> ExactSizeIterator for PixelIter<'a> { }
