extern crate bdf;

use std::env;
use std::process::exit;

fn main() {
	let font      = bdf::open(env::args().nth(1).expect("missing font file")).unwrap();
	let codepoint = env::args().nth(2).expect("missing string").chars().next().expect("missing character");
	let glyph     = font.glyphs().get(&codepoint).unwrap_or_else(|| exit(1));

	if glyph.bounds().y >= 0 {
		for _ in 0 .. (font.bounds().height - glyph.bounds().height) as i32 - glyph.bounds().y + font.bounds().y {
			for _ in 0 .. font.bounds().width {
				print!("░░");
			}

			print!("\n");
		}
	}
	else {
		for _ in 0 .. (font.bounds().height - glyph.bounds().height) as i32 + (font.bounds().y - glyph.bounds().y) {
			for _ in 0 .. font.bounds().width {
				print!("░░");
			}

			print!("\n");
		}
	}

	for y in 0 .. glyph.height() {
		if glyph.bounds().x > 0 {
			for _ in 0 .. glyph.bounds().x {
				print!("░░");
			}
		}

		for x in 0 .. glyph.width() {
			if glyph.get(x, y) {
				print!("██");
			}
			else {
				print!("░░");
			}
		}

		if glyph.bounds().width < font.bounds().width {
			for _ in 0 .. (font.bounds().width - glyph.bounds().width) as i32 - glyph.bounds().x {
				print!("░░");
			}
		}

		print!("\n");
	}

	if glyph.bounds().y >= 0 {
		for _ in 0 .. font.bounds().y.abs() + glyph.bounds().y {
			for _ in 0 .. font.bounds().width {
				print!("░░");
			}

			print!("\n");
		}
	}
	else {
		for _ in 0 .. font.bounds().y.abs() + glyph.bounds().y {
			for _ in 0 .. font.bounds().width {
				print!("░░");
			}

			print!("\n");
		}
	}
}
