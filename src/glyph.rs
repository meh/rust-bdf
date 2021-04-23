use std::ops::{Deref, DerefMut};
use crate::{BoundingBox, Bitmap, Direction};

/// A font glyph.
#[derive(Clone, Debug)]
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
	#[inline]
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
	#[inline]
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
	#[inline]
	pub fn name(&self) -> &str {
		&self.name.as_ref().unwrap().as_ref()
	}

	/// Sets the name.
	#[inline]
	pub fn set_name<T: Into<String>>(&mut self, name: T) {
		self.name = Some(name.into());
	}

	/// Gets the codepoint.
	#[inline]
	pub fn codepoint(&self) -> char {
		self.codepoint.unwrap()
	}

	/// Sets the codepoint.
	#[inline]
	pub fn set_codepoint(&mut self, codepoint: char) {
		self.codepoint = Some(codepoint);
	}

	/// Gets the direction.
	#[inline]
	pub fn direction(&self) -> Direction {
		self.direction
	}

	/// Sets the direction.
	#[inline]
	pub fn set_direction(&mut self, direction: Direction) {
		self.direction = direction;
	}

	/// Gets the scalable width.
	#[inline]
	pub fn scalable_width(&self) -> Option<&(u32, u32)> {
		self.scalable_width.as_ref()
	}

	/// Sets the scalable width.
	#[inline]
	pub fn set_scalable_width(&mut self, value: Option<(u32, u32)>) {
		self.scalable_width = value;
	}

	/// Gets the device width.
	#[inline]
	pub fn device_width(&self) -> Option<&(u32, u32)> {
		self.device_width.as_ref()
	}

	/// Sets the device width.
	#[inline]
	pub fn set_device_width(&mut self, value: Option<(u32, u32)>) {
		self.device_width = value;
	}

	/// Gets the alternate scalable width.
	#[inline]
	pub fn alternate_scalable_width(&self) -> Option<&(u32, u32)> {
		self.alternate_scalable_width.as_ref()
	}

	/// Sets the alternate scalable width.
	#[inline]
	pub fn set_alternate_scalable_width(&mut self, value: Option<(u32, u32)>) {
		self.alternate_scalable_width = value;
	}

	/// Gets the alternate device width.
	#[inline]
	pub fn alternate_device_width(&self) -> Option<&(u32, u32)> {
		self.alternate_device_width.as_ref()
	}

	/// Sets the alternate device width.
	#[inline]
	pub fn set_alternate_device_width(&mut self, value: Option<(u32, u32)>) {
		self.alternate_device_width = value;
	}

	/// Gets the offset vector.
	#[inline]
	pub fn vector(&self) -> Option<&(u32, u32)> {
		self.vector.as_ref()
	}

	/// Sets the offset vector.
	#[inline]
	pub fn set_vector(&mut self, value: Option<(u32, u32)>) {
		self.vector = value;
	}

	/// Gets the bounds.
	#[inline]
	pub fn bounds(&self) -> &BoundingBox {
		self.bounds.as_ref().unwrap()
	}

	/// Sets the bounds.
	#[inline]
	pub fn set_bounds(&mut self, bounds: BoundingBox) {
		self.bounds = Some(bounds);
	}

	/// Gets the bitmap.
	#[inline]
	pub fn map(&self) -> &Bitmap {
		&self.map
	}

	/// Sets the bitmap.
	#[inline]
	pub fn set_map(&mut self, map: Bitmap) {
		self.map = map;
	}

	/// Create an iterator over the pixels which will yield `((x, y), value)`.
	#[inline]
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

	#[inline]
	fn deref(&self) -> &Bitmap {
		&self.map
	}
}

impl DerefMut for Glyph {
	#[inline]
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

	#[inline]
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

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let width   = self.map.width();
		let height  = self.map.height();
		let current = (self.y * height) + self.x;

		(current as usize, Some(((width * height) - current) as usize))
	}
}

impl<'a> ExactSizeIterator for PixelIter<'a> { }
