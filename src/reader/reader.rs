use std::io::{BufRead, BufReader, Lines, Read};
use std::{char, u64};

use crate::{Bitmap, BoundingBox, Direction, Entry, Error, Property};

/// The font reader.
pub struct Reader<T: Read> {
    /// The number of lines that have been processed by this reader so far
    ///
    /// Used in error messages to provide extra context
    line_number: u32,
    stream: Lines<BufReader<T>>,

    default: Option<BoundingBox>,
    current: Option<BoundingBox>,
}

impl<T: Read> From<T> for Reader<T> {
    fn from(stream: T) -> Reader<T> {
        Reader {
            line_number: 0,
            stream: BufReader::new(stream).lines(),

            default: None,
            current: None,
        }
    }
}

macro_rules! parse_int {
    ($e:expr, $line:expr, $line_number:expr) => {
        $e.parse().map_err(|e| Error::Parse {
            error: e,
            line: $line.clone(),
            line_number: $line_number,
        })?
    };
}

impl<T: Read> Reader<T> {
    /// Get the next entry.
    pub fn entry(&mut self) -> Result<Entry, Error> {
        let mut line = String::new();
        while line.is_empty() {
            line = self.stream.next().ok_or(Error::End)??;
            self.line_number += 1;
        }
        let line_number = self.line_number;

        let (id, rest) = match line.find(' ') {
            Some(n) => (&line[0..n], Some((&line[n..]).trim())),

            None => ((&line[..]).trim(), None),
        };

        match id {
            "COMMENT" => {
                if let Some(rest) = rest {
                    Ok(Entry::Comment(crate::property::extract(rest)))
                } else {
                    Ok(Entry::Comment("".to_owned()))
                }
            }

            "STARTFONT" => {
                if let Some(rest) = rest {
                    Ok(Entry::StartFont(rest.to_owned()))
                } else {
                    Err(Error::MissingVersion { line, line_number })
                }
            }

            "FONT" => {
                if let Some(rest) = rest {
                    Ok(Entry::Font(rest.to_owned()))
                } else {
                    Err(Error::MissingValue {
                        property_name: "FONT".to_owned(),
                        line_number,
                    })
                }
            }

            "SIZE" => {
                if let Some(rest) = rest {
                    let split = rest.split(' ').collect::<Vec<_>>();

                    if split.len() != 3 {
                        return Err(Error::MissingValue {
                            property_name: id.to_owned(),
                            line_number,
                        });
                    }

                    Ok(Entry::Size(
                        parse_int!(split[0], line, line_number),
                        parse_int!(split[1], line, line_number),
                        parse_int!(split[2], line, line_number),
                    ))
                } else {
                    Err(Error::MissingValue {
                        property_name: id.to_owned(),
                        line_number,
                    })
                }
            }

            "FONTBOUNDINGBOX" => {
                if let Some(rest) = rest {
                    let split = rest.split(' ').collect::<Vec<_>>();

                    if split.len() != 4 {
                        return Err(Error::MissingValue {
                            property_name: id.to_owned(),
                            line_number,
                        });
                    }

                    let bbx = BoundingBox {
                        width: parse_int!(split[0], line, line_number),
                        height: parse_int!(split[1], line, line_number),

                        x: parse_int!(split[2], line, line_number),
                        y: parse_int!(split[3], line, line_number),
                    };

                    self.default = Some(bbx);

                    Ok(Entry::FontBoundingBox(bbx))
                } else {
                    Err(Error::MissingValue {
                        property_name: id.to_owned(),
                        line_number,
                    })
                }
            }

            "CONTENTVERSION" => {
                if let Some(rest) = rest {
                    Ok(Entry::ContentVersion(rest.to_owned()))
                } else {
                    Err(Error::MissingValue {
                        property_name: id.to_owned(),
                        line_number,
                    })
                }
            }

            "CHARS" => {
                if let Some(rest) = rest {
                    Ok(Entry::Chars(parse_int!(rest, line, line_number)))
                } else {
                    Err(Error::MissingValue {
                        property_name: id.to_owned(),
                        line_number,
                    })
                }
            }

            "STARTCHAR" => {
                if let Some(rest) = rest {
                    Ok(Entry::StartChar(rest.to_owned()))
                } else {
                    Err(Error::MissingValue {
                        property_name: id.to_owned(),
                        line_number,
                    })
                }
            }

            "ENCODING" => {
                if let Some(rest) = rest {
                    Ok(Entry::Encoding(
                        char::from_u32(rest.parse().map_err(|_| Error::InvalidCodepoint {
                            line_number,
                            line: line.clone(),
                        })?)
                        .ok_or(Error::InvalidCodepoint { line_number, line })?,
                    ))
                } else {
                    Err(Error::MissingValue {
                        property_name: id.to_owned(),
                        line_number,
                    })
                }
            }

            "METRICSSET" => {
                if let Some(rest) = rest {
                    match rest {
                        "0" => Ok(Entry::Direction(Direction::Default)),
                        "1" => Ok(Entry::Direction(Direction::Alternate)),
                        "2" => Ok(Entry::Direction(Direction::Both)),
                        _ => Err(Error::MissingValue {
                            property_name: id.to_owned(),
                            line_number,
                        }),
                    }
                } else {
                    Err(Error::MissingValue {
                        property_name: id.to_owned(),
                        line_number,
                    })
                }
            }

            "SWIDTH" => {
                if let Some(rest) = rest {
                    let split = rest.split(' ').collect::<Vec<_>>();

                    if split.len() != 2 {
                        return Err(Error::MissingValue {
                            property_name: id.to_owned(),
                            line_number,
                        });
                    }

                    Ok(Entry::ScalableWidth(
                        parse_int!(split[0], line, line_number),
                        parse_int!(split[1], line, line_number),
                    ))
                } else {
                    Err(Error::MissingValue {
                        property_name: id.to_owned(),
                        line_number,
                    })
                }
            }

            "DWIDTH" => {
                if let Some(rest) = rest {
                    let split = rest.split(' ').collect::<Vec<_>>();

                    if split.len() != 2 {
                        return Err(Error::MissingValue {
                            property_name: id.to_owned(),
                            line_number,
                        });
                    }

                    Ok(Entry::DeviceWidth(
                        parse_int!(split[0], line, line_number),
                        parse_int!(split[1], line, line_number),
                    ))
                } else {
                    Err(Error::MissingValue {
                        property_name: id.to_owned(),
                        line_number,
                    })
                }
            }

            "SWIDTH1" => {
                if let Some(rest) = rest {
                    let split = rest.split(' ').collect::<Vec<_>>();

                    if split.len() != 2 {
                        return Err(Error::MissingValue {
                            property_name: id.to_owned(),
                            line_number,
                        });
                    }

                    Ok(Entry::AlternateScalableWidth(
                        parse_int!(split[0], line, line_number),
                        parse_int!(split[1], line, line_number),
                    ))
                } else {
                    Err(Error::MissingValue {
                        property_name: id.to_owned(),
                        line_number,
                    })
                }
            }

            "DWIDTH1" => {
                if let Some(rest) = rest {
                    let split = rest.split(' ').collect::<Vec<_>>();

                    if split.len() != 2 {
                        return Err(Error::MissingValue {
                            property_name: id.to_owned(),
                            line_number,
                        });
                    }

                    Ok(Entry::AlternateDeviceWidth(
                        parse_int!(split[0], line, line_number),
                        parse_int!(split[1], line, line_number),
                    ))
                } else {
                    Err(Error::MissingValue {
                        property_name: id.to_owned(),
                        line_number,
                    })
                }
            }

            "VVECTOR" => {
                if let Some(rest) = rest {
                    let split = rest.split(' ').collect::<Vec<_>>();

                    if split.len() != 2 {
                        return Err(Error::MissingValue {
                            property_name: id.to_owned(),
                            line_number,
                        });
                    }

                    Ok(Entry::Vector(
                        parse_int!(split[0], line, line_number),
                        parse_int!(split[1], line, line_number),
                    ))
                } else {
                    Err(Error::MissingValue {
                        property_name: id.to_owned(),
                        line_number,
                    })
                }
            }

            "BBX" => {
                if let Some(rest) = rest {
                    let split = rest.split(' ').collect::<Vec<_>>();

                    if split.len() != 4 {
                        return Err(Error::MissingValue {
                            property_name: id.to_owned(),
                            line_number,
                        });
                    }

                    let bbx = BoundingBox {
                        width: parse_int!(split[0], line, line_number),
                        height: parse_int!(split[1], line, line_number),

                        x: parse_int!(split[2], line, line_number),
                        y: parse_int!(split[3], line, line_number),
                    };

                    self.current = Some(bbx);

                    Ok(Entry::BoundingBox(bbx))
                } else {
                    Err(Error::MissingValue {
                        property_name: id.to_owned(),
                        line_number,
                    })
                }
            }

            "BITMAP" => {
                let (width, height) = if let Some(BoundingBox { width, height, .. }) = self.current
                {
                    (width, height)
                } else if let Some(BoundingBox { width, height, .. }) = self.default {
                    (width, height)
                } else {
                    return Err(Error::MissingBoundingBox {
                        line: line.clone(),
                        line_number,
                    });
                };

                let rows = self.stream.by_ref().take(height as usize);
                self.line_number += height;
                let line_number = self.line_number;
                let mut map = Bitmap::new(width, height);

                for (y, row) in rows.into_iter().enumerate() {
                    let row = u64::from_str_radix(row?.as_ref(), 16).map_err(|e| Error::Parse {
                        error: e,
                        line_number,
                        line: line.clone(),
                    })? >> ((8 - (width % 8)) % 8);

                    for x in 0..width {
                        map.set(width - x - 1, y as u32, ((row >> x) & 1) == 1);
                    }
                }

                self.current = None;

                Ok(Entry::Bitmap(map))
            }

            "ENDCHAR" => Ok(Entry::EndChar),

            "ENDFONT" => Ok(Entry::EndFont),

            "STARTPROPERTIES" => {
                if let Some(rest) = rest {
                    Ok(Entry::StartProperties(parse_int!(rest, line, line_number)))
                } else {
                    Err(Error::MissingValue {
                        property_name: id.to_owned(),
                        line_number,
                    })
                }
            }

            "ENDPROPERTIES" => Ok(Entry::EndProperties),

            _ => {
                if let Some(rest) = rest {
                    Ok(Entry::Property(id.to_owned(), Property::parse(rest)))
                } else {
                    Ok(Entry::Unknown(id.to_owned()))
                }
            }
        }
    }
}

impl<T: Read> Iterator for Reader<T> {
    type Item = Entry;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        match self.entry() {
            Ok(entry) => Some(entry),

            Err(..) => None,
        }
    }
}
