#[allow(clippy::module_inception)]
mod writer;
pub use self::writer::Writer;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::{Direction, Entry, Error, Font};

/// Create a `Writer` from a `Write`.
pub fn new<T: Write>(stream: T) -> Writer<T> {
    Writer::from(stream)
}

/// Save the font into a BDF file.
pub fn save<T: AsRef<Path>>(path: T, font: &Font) -> Result<(), Error> {
    write(File::create(path)?, font)
}

/// Write the font to the writer.
pub fn write<T: Write>(stream: T, font: &Font) -> Result<(), Error> {
    if !font.validate() {
        return Err(Error::MalformedFont);
    }

    if font.glyphs().iter().any(|(_, g)| !g.validate()) {
        return Err(Error::MalformedChar);
    }

    let mut writer = new(stream);

    writer.entry(&Entry::StartFont(font.format().to_owned()))?;
    writer.entry(&Entry::Font(font.name().to_owned()))?;
    writer.entry(&Entry::Size(font.size().pt, font.size().x, font.size().y))?;

    if let Some(version) = font.version() {
        writer.entry(&Entry::ContentVersion(version.to_owned()))?;
    }

    writer.entry(&Entry::FontBoundingBox(*font.bounds()))?;

    if font.direction() != Direction::Default {
        writer.entry(&Entry::Direction(font.direction()))?;
    }

    if let Some(&(x, y)) = font.scalable_width() {
        writer.entry(&Entry::ScalableWidth(x, y))?;
    }

    if let Some(&(x, y)) = font.device_width() {
        writer.entry(&Entry::DeviceWidth(x, y))?;
    }

    if let Some(&(x, y)) = font.alternate_scalable_width() {
        writer.entry(&Entry::AlternateScalableWidth(x, y))?;
    }

    if let Some(&(x, y)) = font.alternate_device_width() {
        writer.entry(&Entry::AlternateDeviceWidth(x, y))?;
    }

    if let Some(&(x, y)) = font.vector() {
        writer.entry(&Entry::Vector(x, y))?;
    }

    if !font.properties().is_empty() {
        writer.entry(&Entry::StartProperties(font.properties().len()))?;

        for (name, value) in font.properties() {
            writer.entry(&Entry::Property(name.clone(), value.clone()))?;
        }

        writer.entry(&Entry::EndProperties)?;
    }

    writer.entry(&Entry::Chars(font.glyphs().len()))?;

    for (codepoint, glyph) in font.glyphs() {
        writer.entry(&Entry::StartChar(glyph.name().to_owned()))?;

        writer.entry(&Entry::Encoding(*codepoint))?;

        if glyph.direction() != Direction::Default {
            writer.entry(&Entry::Direction(glyph.direction()))?;
        }

        if let Some(&(x, y)) = glyph.scalable_width() {
            writer.entry(&Entry::ScalableWidth(x, y))?;
        }

        if let Some(&(x, y)) = glyph.device_width() {
            writer.entry(&Entry::DeviceWidth(x, y))?;
        }

        if let Some(&(x, y)) = glyph.alternate_scalable_width() {
            writer.entry(&Entry::AlternateScalableWidth(x, y))?;
        }

        if let Some(&(x, y)) = glyph.alternate_device_width() {
            writer.entry(&Entry::AlternateDeviceWidth(x, y))?;
        }

        if let Some(&(x, y)) = glyph.vector() {
            writer.entry(&Entry::Vector(x, y))?;
        }

        writer.entry(&Entry::BoundingBox(*glyph.bounds()))?;

        writer.entry(&Entry::Bitmap(glyph.map().clone()))?;

        writer.entry(&Entry::EndChar)?;
    }

    writer.entry(&Entry::EndFont)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use crate::{writer, Bitmap, BoundingBox, Direction, Entry, Property};

    pub fn assert(entry: Entry, string: &str) {
        let mut output = Vec::new();

        {
            let mut writer = writer::new(&mut output);
            writer.entry(&entry).unwrap();
        }

        println!("in: {}", from_utf8(&output).unwrap());
        println!("out: {}", &string);

        assert_eq!(from_utf8(&output).unwrap(), string);
    }

    #[test]
    fn start_font() {
        assert(Entry::StartFont("2.2".to_owned()), "STARTFONT 2.2\n");
    }

    #[test]
    fn comment() {
        assert(Entry::Comment("test".to_owned()), "COMMENT \"test\"\n");
    }

    #[test]
    fn content_version() {
        assert(
            Entry::ContentVersion("1.0.0".to_owned()),
            "CONTENTVERSION 1.0.0\n",
        );
    }

    #[test]
    fn font() {
        assert(
            Entry::Font("-Gohu-GohuFont-Bold-R-Normal--11-80-100-100-C-60-ISO10646-1".to_owned()),
            "FONT -Gohu-GohuFont-Bold-R-Normal--11-80-100-100-C-60-ISO10646-1\n",
        );
    }

    #[test]
    fn size() {
        assert(Entry::Size(16, 100, 100), "SIZE 16 100 100\n");
    }

    #[test]
    fn chars() {
        assert(Entry::Chars(42), "CHARS 42\n");
    }

    #[test]
    fn font_bounding_box() {
        assert(
            Entry::FontBoundingBox(BoundingBox {
                width: 6,
                height: 11,
                x: 0,
                y: -2,
            }),
            "FONTBOUNDINGBOX 6 11 0 -2\n",
        );
    }

    #[test]
    fn end_font() {
        assert(Entry::EndFont, "ENDFONT\n");
    }

    #[test]
    fn start_properties() {
        assert(Entry::StartProperties(23), "STARTPROPERTIES 23\n");
    }

    #[test]
    fn property() {
        assert(
            Entry::Property(
                "FOUNDRY".to_owned(),
                Property::String("GohuFont".to_owned()),
            ),
            "FOUNDRY \"GohuFont\"\n",
        );

        assert(
            Entry::Property("X_HEIGHT".to_owned(), Property::Integer(4)),
            "X_HEIGHT 4\n",
        );
    }

    #[test]
    fn end_properties() {
        assert(Entry::EndProperties, "ENDPROPERTIES\n");
    }

    #[test]
    fn start_char() {
        assert(
            Entry::StartChar("<control>".to_owned()),
            "STARTCHAR <control>\n",
        );
    }

    #[test]
    fn encoding() {
        assert(Entry::Encoding('\u{0}'), "ENCODING 0\n");
    }

    #[test]
    fn direction() {
        assert(Entry::Direction(Direction::Default), "METRICSSET 0\n");
        assert(Entry::Direction(Direction::Alternate), "METRICSSET 1\n");
        assert(Entry::Direction(Direction::Both), "METRICSSET 2\n");
    }

    #[test]
    fn scalable_width() {
        assert(Entry::ScalableWidth(392, 0), "SWIDTH 392 0\n");
    }

    #[test]
    fn device_width() {
        assert(Entry::DeviceWidth(6, 0), "DWIDTH 6 0\n");
    }

    #[test]
    fn alternate_scalable_width() {
        assert(Entry::AlternateScalableWidth(392, 0), "SWIDTH1 392 0\n");
    }

    #[test]
    fn alternate_device_width() {
        assert(Entry::AlternateDeviceWidth(6, 0), "DWIDTH1 6 0\n");
    }

    #[test]
    fn vector() {
        assert(Entry::Vector(6, 0), "VVECTOR 6 0\n");
    }

    #[test]
    fn bounding_box() {
        assert(
            Entry::BoundingBox(BoundingBox {
                width: 6,
                height: 11,
                x: 0,
                y: -2,
            }),
            "BBX 6 11 0 -2\n",
        );
    }

    #[test]
    fn bitmap() {
        let mut bitmap = Bitmap::new(6, 11);

        // 00

        // 70
        bitmap.set(1, 1, true);
        bitmap.set(2, 1, true);
        bitmap.set(3, 1, true);

        // D8
        bitmap.set(0, 2, true);
        bitmap.set(1, 2, true);
        bitmap.set(3, 2, true);
        bitmap.set(4, 2, true);

        // D8
        bitmap.set(0, 3, true);
        bitmap.set(1, 3, true);
        bitmap.set(3, 3, true);
        bitmap.set(4, 3, true);

        // F8
        bitmap.set(0, 4, true);
        bitmap.set(1, 4, true);
        bitmap.set(2, 4, true);
        bitmap.set(3, 4, true);
        bitmap.set(4, 4, true);

        // D8
        bitmap.set(0, 5, true);
        bitmap.set(1, 5, true);
        bitmap.set(3, 5, true);
        bitmap.set(4, 5, true);

        // D8
        bitmap.set(0, 6, true);
        bitmap.set(1, 6, true);
        bitmap.set(3, 6, true);
        bitmap.set(4, 6, true);

        // D8
        bitmap.set(0, 7, true);
        bitmap.set(1, 7, true);
        bitmap.set(3, 7, true);
        bitmap.set(4, 7, true);

        // D8
        bitmap.set(0, 8, true);
        bitmap.set(1, 8, true);
        bitmap.set(3, 8, true);
        bitmap.set(4, 8, true);

        // 00

        // 00

        assert(
            Entry::Bitmap(bitmap),
            "BITMAP\n\
			 00\n\
			 70\n\
			 D8\n\
			 D8\n\
			 F8\n\
			 D8\n\
			 D8\n\
			 D8\n\
			 D8\n\
			 00\n\
			 00\n",
        );
    }

    #[test]
    fn end_char() {
        assert(Entry::EndChar, "ENDCHAR\n");
    }

    #[test]
    #[should_panic]
    fn unknown() {
        assert(Entry::Unknown("HUE".to_owned()), "");
    }
}
