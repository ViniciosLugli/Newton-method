pub struct CalculatedFunction {
	pub fx: f64,
	pub dx: f64,
}

impl CalculatedFunction {
	pub fn new(fx: f64, dx: f64) -> CalculatedFunction {
		CalculatedFunction { fx, dx }
	}
}

impl std::fmt::Debug for CalculatedFunction {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Fx: {}, Dx: {}", self.fx, self.dx)
	}
}

