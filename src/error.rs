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
    #[error("{error} on line {line_number}: `{line}`")]
    Parse {
        /// The parser error
        error: num::ParseIntError,
        /// The line number in the font file this was encountered on
        line_number: u32,
        /// The contents of the line that this error was encountered on
        line: String,
    },

    /// `STARTFONT` is missing the format version.
    #[error("Missing version from STARTFONT on line {line_number}: {line}")]
    MissingVersion {
        /// The line number in the font file this was encountered on
        line_number: u32,
        /// The contents of the line that this error was encountered on
        line: String,
    },

    /// There was no bounding box for a character.
    #[error("Missing bounding box on line {line_number}: {line}")]
    MissingBoundingBox {
        /// The line number in the font file this was encountered on
        line_number: u32,
        /// The contents of the line that this error was encountered on
        line: String,
    },

    /// An entry is missing a value.
    #[error("Missing value for property `{property_name}` on line {line_number}")]
    MissingValue {
        /// The name of the property that was missing a value
        property_name: String,
        /// The contents of the line that this error was encountered on
        line_number: u32,
    },

    /// An unknown error.
    #[error("An invalid codepoint has been found")]
    InvalidCodepoint {
        /// The line number in the font file this was encountered on
        line_number: u32,
        /// The contents of the line that this error was encountered on
        line: String,
    },

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
