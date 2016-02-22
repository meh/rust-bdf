use std::ops::{Deref, DerefMut};
use bit_set::BitSet;

/// The bitmap of a glyph.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Bitmap {
	width:  u32,
	height: u32,

	bits: BitSet,
}

impl Default for Bitmap {
	#[inline]
	fn default() -> Self {
		Bitmap::new(0, 0)
	}
}

impl Bitmap {
	/// Creates a bitmap of the given size.
	#[inline]
	pub fn new(width: u32, height: u32) -> Self {
		Bitmap {
			width:  width,
			height: height,

			bits: BitSet::new(),
		}
	}

	/// Gets the width.
	#[inline]
	pub fn width(&self) -> u32 {
		self.width
	}

	/// Gets the height.
	#[inline]
	pub fn height(&self) -> u32 {
		self.height
	}

	/// Gets a bit from the map.
	#[inline]
	pub fn get(&self, x: u32, y: u32) -> bool {
		if y >= self.height || x >= self.width {
			panic!("out of bounds");
		}

		self.bits.contains(((y * self.width + x) as usize))
	}

	/// Sets a bit of the map.
	#[inline]
	pub fn set(&mut self, x: u32, y: u32, value: bool) {
		if y >= self.height || x >= self.width {
			panic!("out of bounds");
		}

		if value {
			self.bits.insert((y * self.width + x) as usize);
		}
		else {
			self.bits.remove(((y * self.width + x) as usize));
		}
	}
}

impl Deref for Bitmap {
	type Target = BitSet;

	#[inline]
	fn deref(&self) -> &BitSet {
		&self.bits
	}
}

impl DerefMut for Bitmap {
	#[inline]
	fn deref_mut(&mut self) -> &mut BitSet {
		&mut self.bits
	}
}
