extern crate bdf;

#[test]
fn open() {
	let font = bdf::open("tests/gohufont.bdf").unwrap();

	assert_eq!(font.format(), "2.1");
}

#[test]
#[should_panic]
fn open_fail() {
	bdf::open("hue").unwrap();
}
