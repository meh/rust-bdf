use std::io::{Write, BufWriter};

use crate::{Error, Entry, Property, Direction};

macro_rules! write {
	($dst:expr, $($arg:tt)*) => (
		$dst.write_all(format!($($arg)*).as_bytes())?
	)
}

/// The font writer.
pub struct Writer<T: Write> {
	stream: BufWriter<T>,
}

impl<T: Write> From<T> for Writer<T> {
	fn from(stream: T) -> Writer<T> {
		Writer {
			stream: BufWriter::new(stream),
		}
	}
}

impl<T: Write> Writer<T> {
	/// Write an entry.
	pub fn entry(&mut self, entry: &Entry) -> Result<(), Error> {
		match entry {
			&Entry::StartFont(ref string) =>
				write!(self.stream, "STARTFONT {}\n", string),

			&Entry::Comment(ref string) =>
				write!(self.stream, "COMMENT \"{}\"\n", string.replace("\"", "\"\"")),

			&Entry::ContentVersion(ref string) =>
				write!(self.stream, "CONTENTVERSION {}\n", string),

			&Entry::Font(ref string) =>
				write!(self.stream, "FONT {}\n", string),

			&Entry::Size(pt, x, y) =>
				write!(self.stream, "SIZE {} {} {}\n", pt, x, y),

			&Entry::Chars(chars) =>
				write!(self.stream, "CHARS {}\n", chars),

			&Entry::FontBoundingBox(ref bbx) =>
				write!(self.stream, "FONTBOUNDINGBOX {} {} {} {}\n", bbx.width, bbx.height, bbx.x, bbx.y),

			&Entry::EndFont =>
				write!(self.stream, "ENDFONT\n"),

			&Entry::StartProperties(len) =>
				write!(self.stream, "STARTPROPERTIES {}\n", len),

			&Entry::Property(ref name, ref value) =>
				match value {
					&Property::String(ref string) =>
						write!(self.stream, "{} \"{}\"\n", name, string.replace("\"", "\"\"")),

					&Property::Integer(value) =>
						write!(self.stream, "{} {}\n", name, value),
				},

			&Entry::EndProperties =>
				write!(self.stream, "ENDPROPERTIES\n"),

			&Entry::StartChar(ref name) =>
				write!(self.stream, "STARTCHAR {}\n", name),

			&Entry::Encoding(value) =>
				write!(self.stream, "ENCODING {}\n", value as u32),

			&Entry::Direction(direction) =>
				match direction {
					Direction::Default =>
						write!(self.stream, "METRICSSET 0\n"),

					Direction::Alternate =>
						write!(self.stream, "METRICSSET 1\n"),

					Direction::Both =>
						write!(self.stream, "METRICSSET 2\n"),
				},

			&Entry::ScalableWidth(x, y) =>
				write!(self.stream, "SWIDTH {} {}\n", x, y),

			&Entry::DeviceWidth(x, y) =>
				write!(self.stream, "DWIDTH {} {}\n", x, y),

			&Entry::AlternateScalableWidth(x, y) =>
				write!(self.stream, "SWIDTH1 {} {}\n", x, y),

			&Entry::AlternateDeviceWidth(x, y) =>
				write!(self.stream, "DWIDTH1 {} {}\n", x, y),

			&Entry::Vector(x, y) =>
				write!(self.stream, "VVECTOR {} {}\n", x, y),

			&Entry::BoundingBox(ref bbx) =>
				write!(self.stream, "BBX {} {} {} {}\n", bbx.width, bbx.height, bbx.x, bbx.y),

			&Entry::Bitmap(ref map) => {
				write!(self.stream, "BITMAP\n");

				for y in 0 .. map.height() {
					let mut value: u64 = 0;

					for x in 0 .. map.width() {
						value <<= 1;
						value  |= map.get(x, y) as u64;
					}

					value <<= (-(map.width() as i32)).rem_euclid(8);

					let hex = format!("{:X}\n", value);

					if (hex.len() - 1) % 2 != 0 {
						write!(self.stream, "0");
					}

					write!(self.stream, "{}", hex);
				}
			},

			&Entry::EndChar =>
				write!(self.stream, "ENDCHAR\n"),

			&Entry::Unknown(..) =>
				unreachable!(),
		}

		Ok(())
	}
}
