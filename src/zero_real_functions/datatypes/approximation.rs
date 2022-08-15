#[derive(Copy, Clone)]
pub struct Approximation {
	pub approximation: f64,
}

impl Approximation {
	pub fn new(approximation: f64) -> Approximation {
		Approximation { approximation }
	}

	pub fn get_value(&self) -> f64 {
		self.approximation
	}
}

impl std::fmt::Debug for Approximation {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Xk: {}", self.approximation)
	}
}
