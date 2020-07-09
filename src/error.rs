use std::io;
use std::num;
use thiserror::Error;

/// Errors for `Reader` and `Writer`.
#[derive(Debug, Error)]
pub enum Error {
	/// A downstream IO error.
	#[error("{0}")]
	IO(#[from] io::Error),

	/// A downstream parsing error.
	#[error("{0}")]
	Parse(#[from] num::ParseIntError),

	/// `STARTFONT` is missing the format version.
	#[error("Missing version from STARTFONT")]
	MissingVersion,

	/// There was no bounding box for a character.
	#[error("Missing bounding box")]
	MissingBoundingBox,

	/// An entry is missing a value.
	#[error("Missing value for property")]
	MissingValue(String),

	/// An unknown error.
	#[error("An invalid codepoint has been found")]
	InvalidCodepoint,

	/// Eof has been reached.
	#[error("End of file reached")]
	End,

	/// The font declaration is malformed.
	#[error("Malformed font definition")]
	MalformedFont,

	/// The property declarations are malformed.
	#[error("Malformed properties definition")]
	MalformedProperties,

	/// The character declaration is malformed.
	#[error("Malformed character definition")]
	MalformedChar,
}
