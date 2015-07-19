mod reader;
pub use self::reader::Reader;

use std::io::Read;
use std::fs::File;
use std::path::Path;

use {Error, Entry, Font, Glyph, font};

/// Create a `Reader` from a `Read`.
pub fn new<T: Read>(stream: T) -> Reader<T> {
	Reader::from(stream)
}

/// Open a BDF file and read it into a `Font`.
pub fn open<T: AsRef<Path>>(path: T) -> Result<Font, Error> {
	read(try!(File::open(path)))
}

/// Read a BDF stream into a `Font`.
pub fn read<T: Read>(stream: T) -> Result<Font, Error> {
	let mut font   = Default::default();
	let mut reader = new(stream);

	let mut in_font  = false;
	let mut in_props = false;
	let mut in_char  = false;

	let mut glyph = Glyph::empty();

	loop {
		let entry = try!(reader.entry());

		if in_font {
			if let Entry::EndFont = entry {
				if in_char {
					return Err(Error::MalformedChar);
				}

				if in_props {
					return Err(Error::MalformedProperties);
				}

				return Ok(font);
			}

			if let Entry::StartProperties(..) = entry {
				if in_char {
					return Err(Error::MalformedChar);
				}

				in_props = true;

				continue;
			}

			if in_props {
				if let Entry::EndProperties = entry {
					in_props = false;

					continue;
				}

				if let Entry::Property(name, value) = entry {
					font.properties_mut().insert(name, value);

					continue;
				}
				else {
					return Err(Error::MalformedProperties);
				}
			}

			if let Entry::StartChar(name) = entry {
				if in_props {
					return Err(Error::MalformedProperties);
				}

				glyph.set_name(name);
				in_char = true;

				continue;
			}

			if in_char {
				if let Entry::EndChar = entry {
					font.glyphs_mut().insert(glyph.codepoint(), glyph);

					in_char = false;
					glyph   = Glyph::empty();

					continue;
				}

				match entry {
					Entry::Encoding(codepoint) =>
						glyph.set_codepoint(codepoint),

					Entry::ScalableWidth(x, y) =>
						glyph.set_scalable_width(x, y),

					Entry::DeviceWidth(x, y) =>
						glyph.set_device_width(x, y),

					Entry::BoundingBox(bbx) =>
						glyph.set_bounds(Some(bbx)),

					Entry::Bitmap(map) =>
						glyph.set_map(map),

					_ =>
						return Err(Error::MalformedChar)
				}

				continue;
			}

			match entry {
				Entry::Comment(comment) =>
					(),

				Entry::ContentVersion(version) =>
					font.set_version(Some(version)),

				Entry::Font(name) =>
					font.set_name(name),

				Entry::Size(pt, x, y) =>
					font.set_size(font::Size { pt: pt, x: x, y: y }),

				Entry::Chars(..) =>
					(),

				Entry::FontBoundingBox(bbx) =>
					font.set_bounds(bbx),

				_ =>
					return Err(Error::MalformedFont)
			}

			continue;
		}

		if let Entry::StartFont(format) = entry {
			font.set_format(format);
			in_font = true;
		}
		else {
			return Err(Error::MalformedFont);
		}
	}
}

#[cfg(test)]
mod tests {
	use {Entry, BoundingBox, Bitmap, Property, reader};

	pub fn assert(string: &str, entry: Entry) {
		let input = reader::new(string.as_bytes()).last().unwrap();

		assert_eq!(input, entry);
	}

	#[test]
	fn start_font() {
		assert("STARTFONT 2.2\n", Entry::StartFont("2.2".to_owned()));
	}

	#[test]
	fn comment() {
		assert("COMMENT \"hue\"\n", Entry::Comment("hue".to_owned()));
	}

	#[test]
	fn content_version() {
		assert("CONTENTVERSION 1.0.0\n", Entry::ContentVersion("1.0.0".to_owned()));
	}

	#[test]
	fn font() {
		assert("FONT -Gohu-GohuFont-Bold-R-Normal--11-80-100-100-C-60-ISO10646-1\n",
			Entry::Font("-Gohu-GohuFont-Bold-R-Normal--11-80-100-100-C-60-ISO10646-1".to_owned()));
	}

	#[test]
	fn size() {
		assert("SIZE 16 100 100\n", Entry::Size(16, 100, 100));
	}

	#[test]
	fn chars() {
		assert("CHARS 42\n", Entry::Chars(42));
	}

	#[test]
	fn font_bounding_box() {
		assert("FONTBOUNDINGBOX 6 11 0 -2\n",
			Entry::FontBoundingBox(BoundingBox { width: 6, height: 11, x: 0, y: -2 }));
	}

	#[test]
	fn end_font() {
		assert("ENDFONT\n", Entry::EndFont);
	}

	#[test]
	fn start_properties() {
		assert("STARTPROPERTIES 23\n", Entry::StartProperties(23));
	}

	#[test]
	fn property() {
		assert("FOUNDRY \"GohuFont\"\n",
			Entry::Property("FOUNDRY".to_owned(), Property::String("GohuFont".to_owned())));

		assert("X_HEIGHT 4\n",
			Entry::Property("X_HEIGHT".to_owned(), Property::Integer(4)));
	}

	#[test]
	fn end_properties() {
		assert("ENDPROPERTIES\n", Entry::EndProperties);
	}

	#[test]
	fn start_char() {
		assert("STARTCHAR <control>\n", Entry::StartChar("<control>".to_owned()));
	}

	#[test]
	fn encoding() {
		assert("ENCODING 0\n", Entry::Encoding('\u{0}'));
	}

	#[test]
	fn scalable_width() {
		assert("SWIDTH 392 0\n", Entry::ScalableWidth(392, 0));
	}

	#[test]
	fn device_width() {
		assert("DWIDTH 6 0\n", Entry::DeviceWidth(6, 0), );
	}

	#[test]
	fn bounding_box() {
		assert("BBX 6 11 0 -2\n",
			Entry::BoundingBox(BoundingBox { width: 6, height: 11, x: 0, y: -2 }));
	}

	#[test]
	fn bitmap() {
		let mut bitmap = Bitmap::new(6, 11);

		// 00

		// 70
		bitmap.set(1, 1, true);
		bitmap.set(2, 1, true);
		bitmap.set(3, 1, true);

		// D8
		bitmap.set(0, 2, true);
		bitmap.set(1, 2, true);
		bitmap.set(3, 2, true);
		bitmap.set(4, 2, true);

		// D8
		bitmap.set(0, 3, true);
		bitmap.set(1, 3, true);
		bitmap.set(3, 3, true);
		bitmap.set(4, 3, true);

		// F8
		bitmap.set(0, 4, true);
		bitmap.set(1, 4, true);
		bitmap.set(2, 4, true);
		bitmap.set(3, 4, true);
		bitmap.set(4, 4, true);

		// D8
		bitmap.set(0, 5, true);
		bitmap.set(1, 5, true);
		bitmap.set(3, 5, true);
		bitmap.set(4, 5, true);

		// D8
		bitmap.set(0, 6, true);
		bitmap.set(1, 6, true);
		bitmap.set(3, 6, true);
		bitmap.set(4, 6, true);

		// D8
		bitmap.set(0, 7, true);
		bitmap.set(1, 7, true);
		bitmap.set(3, 7, true);
		bitmap.set(4, 7, true);

		// D8
		bitmap.set(0, 8, true);
		bitmap.set(1, 8, true);
		bitmap.set(3, 8, true);
		bitmap.set(4, 8, true);

		// 00

		// 00

		assert(
			"BBX 6 11 0 -2\n\
			 BITMAP\n\
			 00\n\
			 70\n\
			 D8\n\
			 D8\n\
			 F8\n\
			 D8\n\
			 D8\n\
			 D8\n\
			 D8\n\
			 00\n\
			 00\n",

			 Entry::Bitmap(bitmap));
	}

	#[test]
	fn end_char() {
		assert("ENDCHAR\n", Entry::EndChar);
	}

	#[test]
	fn unknown() {
		assert("HUE", Entry::Unknown("HUE".to_owned()));
	}
}
