use {BoundingBox, Bitmap};

#[derive(Clone, Debug)]
pub enum Entry {
	StartFont(String),
	Comment(String),
	ContentVersion(String),
	Font(String),
	Size(u16, u16, u16),
	Chars(usize),
	FontBoundingBox(BoundingBox),
	EndFont,

	StartProperties(usize),
	Property(String, String),
	EndProperties,

	StartChar(String),
	Encoding(char),
	ScalableWidth(u32, u32),
	DeviceWidth(u32, u32),
	BoundingBox(BoundingBox),
	Bitmap(Bitmap),
	EndChar,

	Unknown(String),
}
