use std::fmt;
use std::error;
use std::io;
use std::num;

/// Errors for `Reader` and `Writer`.
#[derive(Debug)]
pub enum Error {
	/// A downstream IO error.
	IO(io::Error),

	/// A downstream parsing error.
	Parse(num::ParseIntError),

	/// `STARTFONT` is missing the format version.
	MissingVersion,

	/// There was no bounding box for a character.
	MissingBoundingBox,

	/// An entry is missing a value.
	MissingValue(String),

	/// An unknown error.
	InvalidCodepoint,

	/// Eof has been reached.
	End,

	/// The font declaration is malformed.
	MalformedFont,

	/// The property declarations are malformed.
	MalformedProperties,

	/// The character declaration is malformed.
	MalformedChar,
}

impl From<io::Error> for Error {
	fn from(value: io::Error) -> Self {
		Error::IO(value)
	}
}

impl From<num::ParseIntError> for Error {
	fn from(value: num::ParseIntError) -> Self {
		Error::Parse(value)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		f.write_str(error::Error::description(self))
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		match self {
			&Error::IO(ref err) =>
				err.description(),

			&Error::Parse(ref err) =>
				error::Error::description(err),

			&Error::MissingVersion =>
				"Missing version from STARTFONT",

			&Error::MissingBoundingBox =>
				"Missing bounding box.",

			&Error::MissingValue(..) =>
				"Missing value for property.",

			&Error::InvalidCodepoint =>
				"An invalid codepoint has been found.",

			&Error::End =>
				"End of file reached.",

			&Error::MalformedFont =>
				"Malformed font definition.",

			&Error::MalformedProperties =>
				"Malformed properties definition.",

			&Error::MalformedChar =>
				"Malformed character definition.",
		}
	}
}
