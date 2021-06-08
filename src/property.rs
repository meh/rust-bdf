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
        } else if let Ok(int) = string.parse() {
            Property::Integer(int)
        } else {
            Property::String(string.into())
        }
    }
}

#[inline]
pub fn extract(string: &str) -> String {
    (&string[1..string.len() - 1]).replace("\"\"", "\"")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_property() {
        assert_eq!(
            Property::String("Hello World".into()),
            Property::parse(r#""Hello World""#)
        );
        assert_eq!(
            Property::String("Hello World".into()),
            Property::parse(r#"Hello World"#)
        );
        assert_eq!(Property::Integer(41), Property::parse(r#"41"#));
        assert_eq!(Property::String("41".into()), Property::parse(r#""41""#));
    }
}
