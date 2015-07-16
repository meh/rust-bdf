mod writer;
pub use self::writer::Writer;

use std::io::Write;
use std::fs::File;
use std::path::Path;

use {Error, Font, Entry};

/// Create a `Writer` from a `Write`.
pub fn new<T: Write>(stream: T) -> Writer<T> {
	Writer::from(stream)
}

/// Save the font into a BDF file.
pub fn save<T: AsRef<Path>>(path: T, font: &Font) -> Result<(), Error> {
	write(try!(File::create(path)), font)
}

/// Write the font to the writer.
pub fn write<T: Write>(stream: T, font: &Font) -> Result<(), Error> {
	let mut writer = new(stream);

	try!(writer.entry(&Entry::StartFont(font.format().to_owned())));

	for comment in font.comments() {
		try!(writer.entry(&Entry::Comment(comment.to_owned())));
	}

	try!(writer.entry(&Entry::Font(font.name().to_owned())));
	try!(writer.entry(&Entry::Size(font.size().pt, font.size().x, font.size().y)));

	if let Some(version) = font.version() {
		try!(writer.entry(&Entry::ContentVersion(version.to_owned())));
	}

	if let Some(bbx) = font.bounds() {
		try!(writer.entry(&Entry::FontBoundingBox(bbx.clone())));
	}

	if font.properties().len() > 0 {
		try!(writer.entry(&Entry::StartProperties(font.properties().len())));

		for (name, value) in font.properties() {
			try!(writer.entry(&Entry::Property(name.clone(), value.clone())));
		}

		try!(writer.entry(&Entry::EndProperties));
	}

	try!(writer.entry(&Entry::Chars(font.glyphs().len())));

	for (codepoint, glyph) in font.glyphs() {
		try!(writer.entry(&Entry::StartChar(glyph.name().to_owned())));

		try!(writer.entry(&Entry::Encoding(*codepoint)));
		try!(writer.entry(&Entry::ScalableWidth(glyph.scalable_width().0, glyph.scalable_width().1)));
		try!(writer.entry(&Entry::DeviceWidth(glyph.device_width().0, glyph.device_width().1)));

		if let Some(bbx) = glyph.bounds() {
			try!(writer.entry(&Entry::BoundingBox(bbx.clone())));
		}

		try!(writer.entry(&Entry::Bitmap(glyph.map().clone())));

		try!(writer.entry(&Entry::EndChar));
	}

	try!(writer.entry(&Entry::EndFont));

	Ok(())
}

#[cfg(test)]
mod tests {
	use std::str::from_utf8;

	use {Entry, BoundingBox, Bitmap, Property, writer};

	pub fn assert(entry: &Entry, string: &str) {
		let mut output = Vec::new();

		{
			let mut writer = writer::new(&mut output);
			writer.entry(entry).unwrap();
		}

		assert_eq!(from_utf8(&output).unwrap(), string);
	}

	#[test]
	fn start_font() {
		assert(&Entry::StartFont("2.2".to_owned()), "STARTFONT 2.2\n");
	}

	#[test]
	fn comment() {
		assert(&Entry::Comment("test".to_owned()), "COMMENT \"test\"\n");
	}

	#[test]
	fn content_version() {
		assert(&Entry::ContentVersion("1.0.0".to_owned()), "CONTENTVERSION 1.0.0\n");
	}

	#[test]
	fn font() {
		assert(&Entry::Font("-Gohu-GohuFont-Bold-R-Normal--11-80-100-100-C-60-ISO10646-1".to_owned()),
			"FONT -Gohu-GohuFont-Bold-R-Normal--11-80-100-100-C-60-ISO10646-1\n");
	}

	#[test]
	fn size() {
		assert(&Entry::Size(16, 100, 100), "SIZE 16 100 100\n");
	}

	#[test]
	fn chars() {
		assert(&Entry::Chars(42), "CHARS 42\n");
	}

	#[test]
	fn font_bounding_box() {
		assert(&Entry::FontBoundingBox(BoundingBox { width: 6, height: 11, x: 0, y: -2 }),
			"FONTBOUNDINGBOX 6 11 0 -2\n");
	}

	#[test]
	fn end_font() {
		assert(&Entry::EndFont, "ENDFONT\n");
	}

	#[test]
	fn start_properties() {
		assert(&Entry::StartProperties(23), "STARTPROPERTIES 23\n");
	}

	#[test]
	fn property() {
		assert(&Entry::Property("FOUNDRY".to_owned(), Property::String("GohuFont".to_owned())),
			"FOUNDRY \"GohuFont\"\n");

		assert(&Entry::Property("X_HEIGHT".to_owned(), Property::Integer(4)),
			"X_HEIGHT 4\n");
	}

	#[test]
	fn end_properties() {
		assert(&Entry::EndProperties, "ENDPROPERTIES\n");
	}

	#[test]
	fn start_char() {
		assert(&Entry::StartChar("<control>".to_owned()), "STARTCHAR <control>\n");
	}

	#[test]
	fn encoding() {
		assert(&Entry::Encoding('\u{0}'), "ENCODING 0\n");
	}

	#[test]
	fn scalable_width() {
		assert(&Entry::ScalableWidth(392, 0), "SWIDTH 392 0\n");
	}

	#[test]
	fn device_width() {
		assert(&Entry::DeviceWidth(6, 0), "DWIDTH 6 0\n");
	}

	#[test]
	fn bounding_box() {
		assert(&Entry::BoundingBox(BoundingBox { width: 6, height: 11, x: 0, y: -2 }),
			"BBX 6 11 0 -2\n");
	}

	#[test]
	fn bitmap() {
		let mut bitmap = Bitmap::new(8, 2);
		bitmap.set(0, 0, true);
		bitmap.set(1, 1, true);

		assert(&Entry::Bitmap(bitmap),
			"BITMAP\n\
			 80\n\
			 40\n");
	}

	#[test]
	fn end_char() {
		assert(&Entry::EndChar, "ENDCHAR\n");
	}

	#[test]
	#[should_panic]
	fn unknown() {
		assert(&Entry::Unknown("HUE".to_owned()), "");
	}
}
