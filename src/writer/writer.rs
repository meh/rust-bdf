use std::io::{Write, BufWriter};

use {Error, Entry, Property, Direction};

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
				try!(self.stream.write_all(format!("STARTFONT {}\n", string).as_bytes())),

			&Entry::Comment(ref string) =>
				try!(self.stream.write_all(format!("COMMENT \"{}\"\n", string.replace("\"", "\"\"")).as_bytes())),

			&Entry::ContentVersion(ref string) =>
				try!(self.stream.write_all(format!("CONTENTVERSION {}\n", string).as_bytes())),

			&Entry::Font(ref string) =>
				try!(self.stream.write_all(format!("FONT {}\n", string).as_bytes())),

			&Entry::Size(pt, x, y) =>
				try!(self.stream.write_all(format!("SIZE {} {} {}\n", pt, x, y).as_bytes())),

			&Entry::Chars(chars) =>
				try!(self.stream.write_all(format!("CHARS {}\n", chars).as_bytes())),

			&Entry::FontBoundingBox(ref bbx) =>
				try!(self.stream.write_all(format!("FONTBOUNDINGBOX {} {} {} {}\n",
					bbx.width, bbx.height, bbx.x, bbx.y).as_bytes())),

			&Entry::EndFont =>
				try!(self.stream.write_all(b"ENDFONT\n")),

			&Entry::StartProperties(len) =>
				try!(self.stream.write_all(format!("STARTPROPERTIES {}\n", len).as_bytes())),

			&Entry::Property(ref name, ref value) =>
				match value {
					&Property::String(ref string) =>
						try!(self.stream.write_all(format!("{} \"{}\"\n", name, string.replace("\"", "\"\"")).as_bytes())),

					&Property::Integer(value) =>
						try!(self.stream.write_all(format!("{} {}\n", name, value).as_bytes())),
				},

			&Entry::EndProperties =>
				try!(self.stream.write_all("ENDPROPERTIES\n".as_bytes())),

			&Entry::StartChar(ref name) =>
				try!(self.stream.write_all(format!("STARTCHAR {}\n", name).as_bytes())),

			&Entry::Encoding(value) =>
				try!(self.stream.write_all(format!("ENCODING {}\n", value as u32).as_bytes())),

			&Entry::Direction(direction) =>
				match direction {
					Direction::Default =>
						try!(self.stream.write_all(b"METRICSSET 0\n")),

					Direction::Alternate =>
						try!(self.stream.write_all(b"METRICSSET 1\n")),

					Direction::Both =>
						try!(self.stream.write_all(b"METRICSSET 2\n")),
				},

			&Entry::ScalableWidth(x, y) =>
				try!(self.stream.write_all(format!("SWIDTH {} {}\n", x, y).as_bytes())),

			&Entry::DeviceWidth(x, y) =>
				try!(self.stream.write_all(format!("DWIDTH {} {}\n", x, y).as_bytes())),

			&Entry::AlternateScalableWidth(x, y) =>
				try!(self.stream.write_all(format!("SWIDTH1 {} {}\n", x, y).as_bytes())),

			&Entry::AlternateDeviceWidth(x, y) =>
				try!(self.stream.write_all(format!("DWIDTH1 {} {}\n", x, y).as_bytes())),

			&Entry::Vector(x, y) =>
				try!(self.stream.write_all(format!("VVECTOR {} {}\n", x, y).as_bytes())),

			&Entry::BoundingBox(ref bbx) =>
				try!(self.stream.write_all(format!("BBX {} {} {} {}\n",
					bbx.width, bbx.height, bbx.x, bbx.y).as_bytes())),

			&Entry::Bitmap(ref map) => {
				try!(self.stream.write_all(b"BITMAP\n"));

				for y in 0 .. map.height() {
					let mut value: u64 = 0;

					for x in 0 .. map.width() {
						value <<= 1;
						value  |= if map.get(x, y) { 1 } else { 0 };
					}

					value <<= 8 - (map.width() % 8);

					let hex = format!("{:X}\n", value);

					if (hex.len() - 1) % 2 != 0 {
						try!(self.stream.write_all(b"0"));
					}

					try!(self.stream.write_all(hex.as_bytes()));
				}
			},

			&Entry::EndChar =>
				try!(self.stream.write_all(b"ENDCHAR\n")),

			&Entry::Unknown(..) =>
				unreachable!(),
		}

		Ok(())
	}
}
