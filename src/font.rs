use std::collections::HashMap;

use {Glyph, Property, BoundingBox};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Size {
	pub pt: u16,
	pub x:  u16,
	pub y:  u16,
}

#[derive(Debug)]
pub struct Font {
	format: String,

	name:    String,
	version: Option<String>,

	size:   Size,
	len:    usize,
	bounds: BoundingBox,

	comments:   Vec<String>,
	properties: HashMap<String, Property>,
	glyphs:     HashMap<char, Glyph>,
}

impl Font {
	pub fn empty() -> Self {
		Font {
			format: "2.2".to_owned(),

			name:    String::new(),
			version: None,

			size:   Size { pt: 0, x: 0, y: 0 },
			len:    0,
			bounds: BoundingBox::new(0, 0, 0, 0),

			comments:   Vec::new(),
			properties: HashMap::new(),
			glyphs:     HashMap::new(),
		}
	}

	pub fn new(name: String, version: Option<String>) -> Self {
		Font {
			name:    name,
			version: version,

			.. Font::empty()
		}
	}

	pub fn format(&self) -> &str {
		&self.format
	}

	pub fn set_format(&mut self, format: String) {
		self.format = format;
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn set_name(&mut self, name: String) {
		self.name = name;
	}

	pub fn version(&self) -> Option<&str> {
		self.version.as_ref().map(|v| v.as_ref())
	}

	pub fn set_version(&mut self, version: Option<String>) {
		self.version = version;
	}

	pub fn size(&self) -> &Size {
		&self.size
	}

	pub fn set_size(&mut self, size: Size) {
		self.size = size;
	}

	pub fn len(&self) -> usize {
		self.len
	}

	pub fn set_len(&mut self, len: usize) {
		self.len = len;
	}

	pub fn bounds(&self) -> &BoundingBox {
		&self.bounds
	}

	pub fn set_bounds(&mut self, bounds: BoundingBox) {
		self.bounds = bounds;
	}

	pub fn comments(&self) -> &Vec<String> {
		&self.comments
	}

	pub fn comments_mut(&mut self) -> &mut Vec<String> {
		&mut self.comments
	}

	pub fn properties(&self) -> &HashMap<String, Property> {
		&self.properties
	}

	pub fn properties_mut(&mut self) -> &mut HashMap<String, Property> {
		&mut self.properties
	}

	pub fn glyphs(&self) -> &HashMap<char, Glyph> {
		&self.glyphs
	}

	pub fn glyphs_mut(&mut self) -> &mut HashMap<char, Glyph> {
		&mut self.glyphs
	}
}
