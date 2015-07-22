extern crate bdf;

use std::env;
use std::process::exit;

fn main() {
	let font      = bdf::open(env::args().nth(1).expect("missing font file")).unwrap();
	let codepoint = env::args().nth(2).expect("missing string").chars().next().expect("missing character");
	let glyph     = font.glyphs().get(&codepoint).unwrap_or_else(|| exit(1));

	for _ in 0 .. (font.bounds().height - glyph.bounds().height) as i32 - glyph.bounds().y + font.bounds().y {
		for _ in 0 .. font.bounds().width {
			print!("░░");
		}

		print!("\n");
	}

	for y in 0 .. glyph.height() {
		for _ in 0 .. glyph.bounds().x {
			print!("░░");
		}

		for x in 0 .. glyph.width() {
			if glyph.get(x, y) {
				print!("██");
			}
			else {
				print!("░░");
			}
		}

		for _ in 0 .. (font.bounds().width - glyph.bounds().width) as i32 - glyph.bounds().x {
			print!("░░");
		}

		print!("\n");
	}

	for _ in 0 .. font.bounds().y.abs() + glyph.bounds().y {
		for _ in 0 .. font.bounds().width {
			print!("░░");
		}

		print!("\n");
	}
}
