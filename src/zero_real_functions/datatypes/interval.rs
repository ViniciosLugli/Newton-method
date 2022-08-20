#[derive(Copy, Clone, Debug)]
pub struct Interval {
	pub lower: f64,
	pub upper: f64,
}

impl Interval {
	pub fn new(lower: f64, upper: f64) -> Interval {
		Interval { lower, upper }
	}

	pub fn get_left_border(&self) -> f64 {
		self.lower
	}

	pub fn get_right_border(&self) -> f64 {
		self.upper
	}

	pub fn get_difference(&self) -> f64 {
		self.upper - self.lower
	}
}
