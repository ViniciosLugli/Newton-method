pub enum Order {
	Two,
	Four,
}
impl Order {
	pub fn value(&self) -> f64 {
		match self {
			Order::Two => 2.0,
			Order::Four => 4.0,
		}
	}

	pub fn constant(&self) -> Vec<f64> {
		match self {
			Order::Two => vec![1.0, 4.0, 1.0],
			Order::Four => vec![7.0, 32.0, 12.0, 32.0, 7.0],
		}
	}

	pub fn divisor(&self) -> f64 {
		match self {
			Order::Two => 6.0,
			Order::Four => 90.0,
		}
	}
}
