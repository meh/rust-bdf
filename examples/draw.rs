extern crate bdf;

use std::env;
use std::process::exit;
use std::char;

fn main() {
	let font      = bdf::open(env::args().nth(1).expect("missing font file")).unwrap();
	let codepoint = char::from_u32(env::args().nth(2).expect("missing codepoint").parse().unwrap()).expect("invalid codepoint");
	let glyph     = font.glyphs().get(&codepoint).unwrap_or_else(|| exit(1));

	for y in 0 .. glyph.height() {
		for x in 0 .. glyph.width() {
			if glyph.get(x, y) {
				print!("██");
			}
			else {
				print!("  ");
			}
		}

		print!("\n");
	}
}
