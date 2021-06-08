extern crate bdf;

#[test]
fn open_gohufont_font() {
    let font = bdf::open("tests/gohufont.bdf").unwrap();
    assert_eq!(font.format(), "2.1");
}

#[cfg(feature = "test-remote-fonts")]
#[test]
fn open_cozette_font() {
    // Test the Cozette font
    let font_data = reqwest::blocking::get(
        "https://github.com/slavfox/Cozette/releases/download/v.1.9.3/cozette.bdf",
    )
    .unwrap()
    .text()
    .unwrap();

    let font = bdf::read(font_data.as_bytes()).unwrap();
    assert_eq!(font.format(), "2.1");
}

#[cfg(feature = "test-remote-fonts")]
#[test]
fn open_zpix_font() {
    // Test the zpix font
    let font_data = reqwest::blocking::get(
        "https://github.com/SolidZORO/zpix-pixel-font/releases/download/v3.1.6/zpix.bdf",
    )
    .unwrap()
    .text()
    .unwrap();

    let font = bdf::read(font_data.as_bytes()).unwrap();
    assert_eq!(font.format(), "2.1");
}

#[test]
#[should_panic]
fn open_fail() {
    bdf::open("hue").unwrap();
}
