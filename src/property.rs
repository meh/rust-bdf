/// A `Font` property.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Property {
	///
	String(String),

	///
	Integer(i64),
}

impl Property {
	/// Parse a property string.
	#[inline]
	pub fn parse(string: &str) -> Property {
		if string.starts_with('"') {
			Property::String(extract(string))
		}
		else {
			Property::Integer(string.parse().unwrap())
		}
	}
}

#[inline]
pub fn extract(string: &str) -> String {
	(&string[1 .. string.len() - 1]).replace("\"\"", "\"")
}
