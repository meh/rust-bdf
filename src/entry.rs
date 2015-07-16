use {BoundingBox, Bitmap};

/// The possible entries in BDF.
#[derive(Clone, Debug)]
pub enum Entry {
	/// `STARTFONT` marks the beginning of the font declaration and contains
	/// the BDF version.
	StartFont(String),

	/// `COMMENT` contains the comment body.
	Comment(String),

	/// `CONTENTVERSION` contains the font version.
	ContentVersion(String),

	/// `FONT` contains the font name.
	Font(String),

	/// `SIZE` contains the pt size, X-axis DPI and Y-axis DPI.
	Size(u16, u16, u16),

	/// `CHARS` contains the number of characters stored.
	Chars(usize),

	/// `FONTBOUNDINGBOX` contains the default bounding box.
	FontBoundingBox(BoundingBox),

	/// `ENDFONT` marks the end of the font declaration.
	EndFont,

	/// `STARTPROPERTIES` marks the beginning of the property declarations and
	/// contains the number of properties.
	StartProperties(usize),

	/// Contains the name and value of a property.
	Property(String, String),

	/// `ENDPROPERTIES` marks the end of the property declarations.
	EndProperties,

	/// `STARTCHAR` marks the beginning of the character declaration and contains
	/// the name of the character.
	StartChar(String),

	/// `ENCODING` contains the codepoint for the glyph.
	Encoding(char),

	/// `SWIDTH` contains the scalable width (x, y) of the glyph.
	ScalableWidth(u32, u32),

	/// `DWIDTH` contains the device width (x, y) of the glyph.
	DeviceWidth(u32, u32),

	/// `BBX` contains the bounds for the glyph.
	BoundingBox(BoundingBox),

	/// `BITMAP` contains the bits of the glyph.
	Bitmap(Bitmap),

	/// `ENDCHAR` marks the end of the character declaration.
	EndChar,

	/// Contains the unknown id.
	Unknown(String),
}
