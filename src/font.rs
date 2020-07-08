use std::collections::HashMap;

use crate::{Glyph, Property, BoundingBox, Direction};

/// Size of a font.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Size {
	/// Point size of the font.
	pub pt: u16,

	/// X-axis DPI.
	pub x: u16,

	/// Y-axis DPI.
	pub y: u16,
}

/// A BDF font.
#[derive(Debug)]
pub struct Font {
	format: String,

	name:    Option<String>,
	version: Option<String>,

	size:   Option<Size>,
	bounds: Option<BoundingBox>,

	direction: Direction,

	scalable_width: Option<(u32, u32)>,
	device_width:   Option<(u32, u32)>,

	alternate_scalable_width: Option<(u32, u32)>,
	alternate_device_width:   Option<(u32, u32)>,

	vector: Option<(u32, u32)>,

	properties: HashMap<String, Property>,
	glyphs:     HashMap<char, Glyph>,
}

impl Default for Font {
	#[inline]
	fn default() -> Self {
		Font {
			format: "2.2".to_owned(),

			name:    None,
			version: None,

			size:   None,
			bounds: None,

			direction: Default::default(),

			scalable_width: None,
			device_width:   None,

			alternate_scalable_width: None,
			alternate_device_width:   None,

			vector: None,

			properties: HashMap::new(),
			glyphs:     HashMap::new(),
		}
	}
}

impl Font {
	/// Create a new font with the given name and content-version.
	#[inline]
	pub fn new<T: Into<String>>(name: T, version: Option<T>) -> Self {
		Font {
			name:    Some(name.into()),
			version: version.map(|v| v.into()),

			.. Default::default()
		}
	}

	/// Validates the definition.
	pub fn validate(&self) -> bool {
		if self.name.is_none() {
			return false;
		}

		if self.size.is_none() {
			return false;
		}

		if self.bounds.is_none() {
			return false;
		}

		true
	}

	/// Gets BDF format version.
	#[inline]
	pub fn format(&self) -> &str {
		&self.format
	}

	/// Sets the BDF format version.
	#[inline]
	pub fn set_format<T: Into<String>>(&mut self, format: T) {
		self.format = format.into();
	}

	/// Gets the name.
	#[inline]
	pub fn name(&self) -> &str {
		self.name.as_ref().unwrap().as_ref()
	}

	/// Sets the name.
	#[inline]
	pub fn set_name<T: Into<String>>(&mut self, name: T) {
		self.name = Some(name.into());
	}

	/// Gets the content-version.
	#[inline]
	pub fn version(&self) -> Option<&str> {
		self.version.as_ref().map(|v| v.as_ref())
	}

	/// Sets the content-version.
	#[inline]
	pub fn set_version<T: Into<String>>(&mut self, version: Option<T>) {
		self.version = version.map(|v| v.into());
	}

	/// Gets the size.
	#[inline]
	pub fn size(&self) -> &Size {
		self.size.as_ref().unwrap()
	}

	/// Sets the size.
	#[inline]
	pub fn set_size(&mut self, size: Size) {
		self.size = Some(size);
	}

	/// Gets the default bounding box.
	#[inline]
	pub fn bounds(&self) -> &BoundingBox {
		self.bounds.as_ref().unwrap()
	}

	/// Sets the default bounding box.
	#[inline]
	pub fn set_bounds(&mut self, bounds: BoundingBox) {
		self.bounds = Some(bounds);
	}

	/// Gets the default direction.
	#[inline]
	pub fn direction(&self) -> Direction {
		self.direction
	}

	/// Sets the default direction.
	#[inline]
	pub fn set_direction(&mut self, direction: Direction) {
		self.direction = direction;
	}

	/// Gets the default scalable width.
	#[inline]
	pub fn scalable_width(&self) -> Option<&(u32, u32)> {
		self.scalable_width.as_ref()
	}

	/// Sets the default scalable width.
	#[inline]
	pub fn set_scalable_width(&mut self, value: Option<(u32, u32)>) {
		self.scalable_width = value;
	}

	/// Gets the default device width.
	#[inline]
	pub fn device_width(&self) -> Option<&(u32, u32)> {
		self.device_width.as_ref()
	}

	/// Sets the default device width.
	#[inline]
	pub fn set_device_width(&mut self, value: Option<(u32, u32)>) {
		self.device_width = value;
	}

	/// Gets the default alternate scalable width.
	#[inline]
	pub fn alternate_scalable_width(&self) -> Option<&(u32, u32)> {
		self.alternate_scalable_width.as_ref()
	}

	/// Sets the default alternate scalable width.
	#[inline]
	pub fn set_alternate_scalable_width(&mut self, value: Option<(u32, u32)>) {
		self.alternate_scalable_width = value;
	}

	/// Gets the default alternate device width.
	#[inline]
	pub fn alternate_device_width(&self) -> Option<&(u32, u32)> {
		self.alternate_device_width.as_ref()
	}

	/// Sets the default alternate device width.
	#[inline]
	pub fn set_alternate_device_width(&mut self, value: Option<(u32, u32)>) {
		self.alternate_device_width = value;
	}

	/// Gets the default offset vector.
	#[inline]
	pub fn vector(&self) -> Option<&(u32, u32)> {
		self.vector.as_ref()
	}

	/// Sets the default offset vector.
	#[inline]
	pub fn set_vector(&mut self, value: Option<(u32, u32)>) {
		self.vector = value;
	}

	/// Gets the properties.
	#[inline]
	pub fn properties(&self) -> &HashMap<String, Property> {
		&self.properties
	}

	/// Gets a mutable reference to the properties.
	#[inline]
	pub fn properties_mut(&mut self) -> &mut HashMap<String, Property> {
		&mut self.properties
	}

	/// Gets the glyphs.
	#[inline]
	pub fn glyphs(&self) -> &HashMap<char, Glyph> {
		&self.glyphs
	}

	/// Gets a mutable reference to the glyphs.
	#[inline]
	pub fn glyphs_mut(&mut self) -> &mut HashMap<char, Glyph> {
		&mut self.glyphs
	}
}
