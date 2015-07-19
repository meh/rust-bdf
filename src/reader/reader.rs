use std::io::{Read, BufRead, BufReader, Lines};
use std::{char, u64};

use {Error, Entry, BoundingBox, Bitmap, Property, Direction};

/// The font reader.
pub struct Reader<T: Read> {
	stream: Lines<BufReader<T>>,

	default: Option<BoundingBox>,
	current: Option<BoundingBox>,
}

impl<T: Read> From<T> for Reader<T> {
	fn from(stream: T) -> Reader<T> {
		Reader {
			stream: BufReader::new(stream).lines(),

			default: None,
			current: None,
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

		match id {
			"COMMENT" => {
				if let Some(rest) = rest {
					Ok(Entry::Comment(::property::extract(rest)))
				}
				else {
					Ok(Entry::Comment("".to_owned()))
				}
			},

			"STARTFONT" => {
				if let Some(rest) = rest {
					Ok(Entry::StartFont(rest.to_owned()))
				}
				else {
					Err(Error::MissingVersion)
				}
			},

			"FONT" => {
				if let Some(rest) = rest {
					Ok(Entry::Font(rest.to_owned()))
				}
				else {
					Err(Error::MissingValue("FONT".to_owned()))
				}
			},

			"SIZE" => {
				if let Some(rest) = rest {
					let split = rest.split(' ').collect::<Vec<_>>();

					if split.len() != 3 {
						return Err(Error::MissingValue(id.to_owned()));
					}

					Ok(Entry::Size(
						try!(split[0].parse()),
						try!(split[1].parse()),
						try!(split[2].parse())))
				}
				else {
					Err(Error::MissingValue(id.to_owned()))
				}
			},

			"FONTBOUNDINGBOX" => {
				if let Some(rest) = rest {
					let split = rest.split(' ').collect::<Vec<_>>();

					if split.len() != 4 {
						return Err(Error::MissingValue(id.to_owned()));
					}

					let bbx = BoundingBox {
						width:  try!(split[0].parse()),
						height: try!(split[1].parse()),

						x: try!(split[2].parse()),
						y: try!(split[3].parse())
					};

					self.default = Some(bbx);

					Ok(Entry::FontBoundingBox(bbx))
				}
				else {
					Err(Error::MissingValue(id.to_owned()))
				}
			},

			"CONTENTVERSION" => {
				if let Some(rest) = rest {
					Ok(Entry::ContentVersion(rest.to_owned()))
				}
				else {
					Err(Error::MissingValue(id.to_owned()))
				}
			},

			"CHARS" => {
				if let Some(rest) = rest {
					Ok(Entry::Chars(try!(rest.parse())))
				}
				else {
					Err(Error::MissingValue(id.to_owned()))
				}
			},

			"STARTCHAR" => {
				if let Some(rest) = rest {
					Ok(Entry::StartChar(rest.to_owned()))
				}
				else {
					Err(Error::MissingValue(id.to_owned()))
				}
			},

			"ENCODING" => {
				if let Some(rest) = rest {
					Ok(Entry::Encoding(
						try!(char::from_u32(try!(rest.parse())).ok_or(Error::InvalidCodepoint))))
				}
				else {
					Err(Error::MissingValue(id.to_owned()))
				}
			},

			"METRICSSET" => {
				if let Some(rest) = rest {
					match rest {
						"0" => Ok(Entry::Direction(Direction::Default)),
						"1" => Ok(Entry::Direction(Direction::Alternate)),
						"2" => Ok(Entry::Direction(Direction::Both)),
						 _  => Err(Error::MissingValue(id.to_owned())),
					}
				}
				else {
					Err(Error::MissingValue(id.to_owned()))
				}
			},

			"SWIDTH" => {
				if let Some(rest) = rest {
					let split = rest.split(' ').collect::<Vec<_>>();

					if split.len() != 2 {
						return Err(Error::MissingValue(id.to_owned()));
					}

					Ok(Entry::ScalableWidth(
						try!(split[0].parse()),
						try!(split[1].parse())))
				}
				else {
					Err(Error::MissingValue(id.to_owned()))
				}
			},

			"DWIDTH" => {
				if let Some(rest) = rest {
					let split = rest.split(' ').collect::<Vec<_>>();

					if split.len() != 2 {
						return Err(Error::MissingValue(id.to_owned()));
					}

					Ok(Entry::DeviceWidth(
						try!(split[0].parse()),
						try!(split[1].parse())))
				}
				else {
					Err(Error::MissingValue(id.to_owned()))
				}
			},

			"SWIDTH1" => {
				if let Some(rest) = rest {
					let split = rest.split(' ').collect::<Vec<_>>();

					if split.len() != 2 {
						return Err(Error::MissingValue(id.to_owned()));
					}

					Ok(Entry::AlternateScalableWidth(
						try!(split[0].parse()),
						try!(split[1].parse())))
				}
				else {
					Err(Error::MissingValue(id.to_owned()))
				}
			},

			"DWIDTH1" => {
				if let Some(rest) = rest {
					let split = rest.split(' ').collect::<Vec<_>>();

					if split.len() != 2 {
						return Err(Error::MissingValue(id.to_owned()));
					}

					Ok(Entry::AlternateDeviceWidth(
						try!(split[0].parse()),
						try!(split[1].parse())))
				}
				else {
					Err(Error::MissingValue(id.to_owned()))
				}
			},

			"VVECTOR" => {
				if let Some(rest) = rest {
					let split = rest.split(' ').collect::<Vec<_>>();

					if split.len() != 2 {
						return Err(Error::MissingValue(id.to_owned()));
					}

					Ok(Entry::Vector(
						try!(split[0].parse()),
						try!(split[1].parse())))
				}
				else {
					Err(Error::MissingValue(id.to_owned()))
				}
			},

			"BBX" => {
				if let Some(rest) = rest {
					let split = rest.split(' ').collect::<Vec<_>>();

					if split.len() != 4 {
						return Err(Error::MissingValue(id.to_owned()));
					}

					let bbx = BoundingBox {
						width: try!(split[0].parse()),
						height: try!(split[1].parse()),

						x: try!(split[2].parse()),
						y: try!(split[3].parse())
					};

					self.current = Some(bbx);

					Ok(Entry::BoundingBox(bbx))
				}
				else {
					Err(Error::MissingValue(id.to_owned()))
				}
			},

			"BITMAP" => {
				let (width, height) = if let Some(BoundingBox { width, height, .. }) = self.current {
					(width, height)
				}
				else if let Some(BoundingBox { width, height, .. }) = self.default {
					(width, height)
				}
				else {
					return Err(Error::MissingBoundingBox);
				};

				let     rows = self.stream.by_ref().take(height as usize).collect::<Vec<_>>();
				let mut map  = Bitmap::new(width, height);

				for (y, row) in rows.into_iter().enumerate() {
					let row = try!(u64::from_str_radix(try!(row).as_ref(), 16)) >> (8 - (width % 8));

					for x in 0 .. width {
						map.set(width - x - 1, y as u32, ((row >> x) & 1) == 1);
					}
				}

				self.current = None;

				Ok(Entry::Bitmap(map))
			},

			"ENDCHAR" => {
				Ok(Entry::EndChar)
			},

			"ENDFONT" => {
				Ok(Entry::EndFont)
			},

			"STARTPROPERTIES" => {
				if let Some(rest) = rest {
					Ok(Entry::StartProperties(try!(rest.parse())))
				}
				else {
					Err(Error::MissingValue(id.to_owned()))
				}
			},

			"ENDPROPERTIES" => {
				Ok(Entry::EndProperties)
			},

			_ => {
				if let Some(rest) = rest {
					Ok(Entry::Property(id.to_owned(), Property::parse(rest)))
				}
				else {
					Ok(Entry::Unknown(id.to_owned()))
				}
			},
		}
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
