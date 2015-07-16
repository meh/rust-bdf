mod reader;
pub use self::reader::Reader;

use std::io::Read;
use std::fs::File;
use std::path::Path;

use {Error, Entry, Font, Property, Glyph, font};

pub fn open<T: AsRef<Path>>(path: T) -> Result<Font, Error> {
	read(try!(File::open(path.as_ref())))
}

pub fn read<T: Read>(mut reader: T) -> Result<Font, Error> {
	let mut font   = Font::empty();
	let mut reader = Reader::from(&mut reader);

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
					font.properties_mut().insert(name, Property::parse(&value));

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
						glyph.set_bounds(bbx),

					Entry::Bitmap(map) =>
						glyph.set_map(map),

					_ =>
						return Err(Error::MalformedChar)
				}

				continue;
			}

			match entry {
				Entry::Comment(comment) =>
					font.comments_mut().push(comment),

				Entry::ContentVersion(version) =>
					font.set_version(Some(version)),

				Entry::Font(name) =>
					font.set_name(name),

				Entry::Size(pt, x, y) =>
					font.set_size(font::Size { pt: pt, x: x, y: y }),

				Entry::Chars(len) =>
					font.set_len(len),

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
