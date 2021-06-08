//! BDF font handler
//!
//! This crate allows you to read and write BDF fonts in Rust.
//!
//! # Example
//!
//! This example will draw a given glyph in your terminal using the given font.
//!
//! ```rust,no_run
//! use std::char;
//! use std::env;
//! use std::process::exit;
//!
//! let font = bdf::open(env::args().nth(1).expect("missing font file")).unwrap();
//! let codepoint = char::from_u32(
//!     env::args()
//!         .nth(2)
//!         .expect("missing codepoint")
//!         .parse()
//!         .unwrap(),
//! )
//! .expect("invalid codepoint");
//! let glyph = font.glyphs().get(&codepoint).unwrap_or_else(|| exit(1));
//!
//! for y in 0..glyph.height() {
//!     for x in 0..glyph.width() {
//!         if glyph.get(x, y) {
//!             print!("██");
//!         } else {
//!             print!("  ");
//!         }
//!     }
//!     print!("\n");
//! }
//! ```

#![warn(missing_docs)]

extern crate bit_set;

mod property;
pub use self::property::Property;

mod glyph;
pub use self::glyph::Glyph;

mod bounding_box;
pub use self::bounding_box::BoundingBox;

mod direction;
pub use self::direction::Direction;

mod bitmap;
pub use self::bitmap::Bitmap;

mod font;
pub use self::font::*;

mod entry;
pub use self::entry::Entry;

mod error;
pub use self::error::Error;

mod reader;
pub use self::reader::{open, read, Reader};

mod writer;
pub use self::writer::{save, write, Writer};
