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
fn strip_quotes(string: &str) -> Option<&str> {
    string.trim().strip_prefix('"')?.strip_suffix('"')
}

#[inline]
pub fn extract(string: &str) -> String {
    match strip_quotes(string) {
        Some(s) => s.replace("\"\"", "\""),
        None => string.to_owned(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extract_string() {
        // no surrounding quotes
        assert_eq!(
            "test".to_string(),
            extract(r#"test"#)
        );
        assert_eq!(
            r#"test"""#.to_string(),
            extract(r#"test"""#)
        );
        // surrounding quotes
        assert_eq!(
            "hello".to_string(),
            extract(r#""hello""#)
        );
        assert_eq!(
            r#"this is a "test""#.to_string(),
            extract(r#""this is a ""test""""#)
        );
    }

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
        assert_eq!(
            Property::String(r#""Hello"World""#.into()),
            Property::parse(r#"""Hello""World"""#)
        );
        assert_eq!(Property::Integer(41), Property::parse(r#"41"#));
        assert_eq!(Property::String("41".into()), Property::parse(r#""41""#));
    }
}
