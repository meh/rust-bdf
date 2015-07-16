use bit_set::BitSet;

#[derive(Clone, Debug)]
pub struct Bitmap {
	width:  u32,
	height: u32,

	bits: BitSet,
}

impl Bitmap {
	pub fn empty() -> Self {
		Bitmap::new(0, 0)
	}

	pub fn new(width: u32, height: u32) -> Self {
		Bitmap {
			width:  width,
			height: height,

			bits: BitSet::new(),
		}
	}

	pub fn width(&self) -> u32 {
		self.width
	}

	pub fn height(&self) -> u32 {
		self.height
	}

	pub fn get(&self, x: u32, y: u32) -> bool {
		self.bits.contains(&((x * self.width + y) as usize))
	}

	pub fn set(&mut self, x: u32, y: u32, value: bool) {
		if value {
			self.bits.insert((x * self.width + y) as usize);
		}
		else {
			self.bits.remove(&((x * self.width + y) as usize));
		}
	}
}
