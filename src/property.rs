#[derive(Debug)]
pub enum Property {
	String(String),
	Integer(i64),
	Float(f32),
}

impl Property {
	pub fn parse(string: &str) -> Property {
		if string.starts_with('"') {
			let string = &string[1..string.len()];

			Property::String(string.to_owned())
		}
		else {
			Property::Integer(string.parse().unwrap())
		}
	}
}
