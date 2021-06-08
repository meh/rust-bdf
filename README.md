# bdf

[![Build & Test](https://github.com/meh/rust-bdf/actions/workflows/rust.yml/badge.svg)](https://github.com/meh/rust-bdf/actions/workflows/rust.yml)

BDF font handler

This crate allows you to read and write BDF fonts in Rust.


## Example

This example will draw a given glyph in your terminal using the given font.


```rust
use std::char;
use std::env;
use std::process::exit;

let font = bdf::open(env::args().nth(1).expect("missing font file")).unwrap();
let codepoint = char::from_u32(
    env::args()
        .nth(2)
        .expect("missing codepoint")
        .parse()
        .unwrap(),
)
.expect("invalid codepoint");
let glyph = font.glyphs().get(&codepoint).unwrap_or_else(|| exit(1));

for y in 0..glyph.height() {
    for x in 0..glyph.width() {
        if glyph.get(x, y) {
            print!("██");
        } else {
            print!("  ");
        }
    }
    print!("\n");
}
```

