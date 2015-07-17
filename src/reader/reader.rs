use std::io::{Read, BufRead, BufReader, Lines};
use std::{char, u64};

use {Error, Entry, BoundingBox, Bitmap, Property};

/// The font reader.
pub struct Reader<T: Read> {
	stream: Lines<BufReader<T>>,

	default_width:  usize,
	default_height: usize,

	width:  usize,
	height: usize,
}

impl<T: Read> From<T> for Reader<T> {
	fn from(stream: T) -> Reader<T> {
		Reader {
			stream: BufReader::with_capacity(1024, stream).lines(),

			default_width:  0,
			default_height: 0,

			width:  0,
			height: 0,
		}
	}
}

impl<T: Read> Reader<T> {
	/// Get the next entry.
	pub fn entry(&mut self) -> Result<Entry, Error> {
		let line = try!(try!(self.stream.next().ok_or(Error::End)));

		let (id, rest) = match line.find(' ') {
			Some(n) =>
				(&line[0..n], Some((&line[n..]).trim())),

			None =>
				((&line[..]).trim(), None)
		};

		if id == "STARTFONT" {
			if let Some(rest) = rest {
				return Ok(Entry::StartFont(rest.to_owned()))
			}

			return Err(Error::MissingVersion);
		}

		if id == "COMMENT" {
			if let Some(rest) = rest {
				return Ok(Entry::Comment((&rest[1..rest.len()-1]).to_owned()));
			}
			else {
				return Ok(Entry::Comment("".to_owned()));
			}
		}

		if id == "FONT" {
			if let Some(rest) = rest {
				return Ok(Entry::Font(rest.to_owned()));
			}

			return Err(Error::MissingValue("FONT".to_owned()))
		}

		if id == "SIZE" {
			if let Some(rest) = rest {
				let split = rest.split(' ').collect::<Vec<_>>();

				if split.len() != 3 {
					return Err(Error::MissingValue(id.to_owned()));
				}

				return Ok(Entry::Size(
					try!(split[0].parse()),
					try!(split[1].parse()),
					try!(split[2].parse())));
			}

			return Err(Error::MissingValue(id.to_owned()));
		}

		if id == "FONTBOUNDINGBOX" {
			if let Some(rest) = rest {
				let split = rest.split(' ').collect::<Vec<_>>();

				if split.len() != 4 {
					return Err(Error::MissingValue(id.to_owned()));
				}

				self.default_width  = try!(split[0].parse());
				self.default_height = try!(split[1].parse());

				return Ok(Entry::FontBoundingBox(BoundingBox {
					width:  try!(split[0].parse()),
					height: try!(split[1].parse()),

					x: try!(split[2].parse()),
					y: try!(split[3].parse())
				}));
			}

			return Err(Error::MissingValue(id.to_owned()));
		}

		if id == "CONTENTVERSION" {
			if let Some(rest) = rest {
				return Ok(Entry::ContentVersion(rest.to_owned()));
			}

			return Err(Error::MissingValue(id.to_owned()));
		}

		if id == "CHARS" {
			if let Some(rest) = rest {
				return Ok(Entry::Chars(try!(rest.parse())));
			}

			return Err(Error::MissingValue(id.to_owned()));
		}

		if id == "STARTCHAR" {
			if let Some(rest) = rest {
				return Ok(Entry::StartChar(rest.to_owned()));
			}

			return Err(Error::MissingValue(id.to_owned()));
		}

		if id == "ENCODING" {
			if let Some(rest) = rest {
				return Ok(Entry::Encoding(
					try!(char::from_u32(try!(rest.parse())).ok_or(Error::Unknown))));
			}

			return Err(Error::MissingValue(id.to_owned()));
		}

		if id == "SWIDTH" {
			if let Some(rest) = rest {
				let split = rest.split(' ').collect::<Vec<_>>();

				if split.len() != 2 {
					return Err(Error::MissingValue(id.to_owned()));
				}

				return Ok(Entry::ScalableWidth(
					try!(split[0].parse()),
					try!(split[1].parse())));
			}

			return Err(Error::MissingValue(id.to_owned()));
		}

		if id == "DWIDTH" {
			if let Some(rest) = rest {
				let split = rest.split(' ').collect::<Vec<_>>();

				if split.len() != 2 {
					return Err(Error::MissingValue(id.to_owned()));
				}

				return Ok(Entry::DeviceWidth(
					try!(split[0].parse()),
					try!(split[1].parse())));
			}

			return Err(Error::MissingValue(id.to_owned()));
		}

		if id == "BBX" {
			if let Some(rest) = rest {
				let split = rest.split(' ').collect::<Vec<_>>();

				if split.len() != 4 {
					return Err(Error::MissingValue(id.to_owned()));
				}

				self.width  = try!(split[0].parse());
				self.height = try!(split[1].parse());

				return Ok(Entry::BoundingBox(BoundingBox {
					width: try!(split[0].parse()),
					height: try!(split[1].parse()),

					x: try!(split[2].parse()),
					y: try!(split[3].parse())
				}));
			}

			return Err(Error::MissingValue(id.to_owned()));
		}

		if id == "BITMAP" {
			if (self.default_width == 0 && self.width == 0) || (self.default_height == 0 && self.height == 0) {
				return Err(Error::MissingValue(id.to_owned()));
			}

			let width  = if self.default_width == 0 { self.width } else { self.default_width };
			let height = if self.default_height == 0 { self.height } else { self.default_height };

			let     rows = self.stream.by_ref().take(height).collect::<Vec<_>>();
			let mut map  = Bitmap::new(width as u32, height as u32);

			for (y, row) in rows.into_iter().enumerate() {
				let row = try!(u64::from_str_radix(try!(row).as_ref(), 16)) >> (8 - (width % 8));

				for x in 0 .. width {
					map.set((width - x - 1) as u32, y as u32, ((row >> x) & 1) == 1);
				}
			}

			return Ok(Entry::Bitmap(map));
		}

		if id == "ENDCHAR" {
			return Ok(Entry::EndChar);
		}

		if id == "ENDFONT" {
			return Ok(Entry::EndFont);
		}

		if id == "STARTPROPERTIES" {
			if let Some(rest) = rest {
				return Ok(Entry::StartProperties(try!(rest.parse())));
			}

			return Err(Error::MissingValue(id.to_owned()));
		}

		if id == "ENDPROPERTIES" {
			return Ok(Entry::EndProperties);
		}

		if let Some(rest) = rest {
			return Ok(Entry::Property(id.to_owned(), Property::parse(rest)));
		}

		Ok(Entry::Unknown(id.to_owned()))
	}
}

impl<T: Read> Iterator for Reader<T> {
	type Item = Entry;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		match self.entry() {
			Ok(entry) =>
				Some(entry),

			Err(Error::End) =>
				None,

			Err(error) =>
				panic!("{}", error)
		}
	}
}
